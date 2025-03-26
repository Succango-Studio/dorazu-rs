#![allow(unexpected_cfgs)]
use std::ffi::CStr;
use std::os::raw::c_char;

use cocoa::base::nil;
use cocoa::foundation::NSString;
use objc::rc::autoreleasepool;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::model::drag_types::{DragData, RemoteImage, RichContent};

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    pub static NSPasteboardNameDrag: *const Object;
}

// 获取拖拽粘贴板的 changeCount
pub fn get_drag_pasteboard_change_count() -> i64 {
    unsafe {
        autoreleasepool(|| {
            let cls = class!(NSPasteboard);
            let pasteboard: *mut Object = msg_send![cls, pasteboardWithName: NSPasteboardNameDrag];

            if pasteboard.is_null() {
                println!("Failed to get NSPasteboard!");
                return 0;
            }

            let count: i64 = msg_send![pasteboard, changeCount];
            count
        })
    }
}

// 获取拖拽粘贴板中的有效数据
pub fn get_drag_pasteboard_data() -> Option<DragData> {
    unsafe {
        autoreleasepool(|| {
            let cls = class!(NSPasteboard);
            let pasteboard: *mut Object = msg_send![cls, pasteboardWithName: NSPasteboardNameDrag];
            if pasteboard.is_null() {
                return None;
            }

            // 处理文件类型数据
            let filenames_ns = NSString::alloc(nil).init_str("NSFilenamesPboardType");
            let file_list: *mut Object = msg_send![pasteboard, propertyListForType: filenames_ns];
            if !file_list.is_null() {
                let count: usize = msg_send![file_list, count];
                let mut paths = Vec::with_capacity(count);
                for i in 0..count {
                    let file_ns: *mut Object = msg_send![file_list, objectAtIndex: i];
                    let utf8_ptr: *const c_char = msg_send![file_ns, UTF8String];
                    if !utf8_ptr.is_null() {
                        let file_str = CStr::from_ptr(utf8_ptr).to_str().unwrap_or("");
                        paths.push(std::path::PathBuf::from(file_str));
                    }
                }
                return Some(DragData::LocalFile(paths));
            }

            // 处理富文本类型数据（HTML）
            let html_type = NSString::alloc(nil).init_str("public.html");
            let html_content: *mut Object = msg_send![pasteboard, stringForType: html_type];
            if !html_content.is_null() {
                let html_ptr: *const c_char = msg_send![html_content, UTF8String];
                let html_str = CStr::from_ptr(html_ptr).to_str().unwrap_or("").to_owned();

                // 尝试获取纯文本作为fallback
                let plain_type = NSString::alloc(nil).init_str("public.utf8-plain-text");
                let plain_content: *mut Object = msg_send![pasteboard, stringForType: plain_type];
                let plain_str = if !plain_content.is_null() {
                    let plain_ptr: *const c_char = msg_send![plain_content, UTF8String];
                    CStr::from_ptr(plain_ptr).to_str().unwrap_or("").to_owned()
                } else {
                    String::new()
                };

                // 远程图片识别逻辑
                let is_single_image = html_str.contains("<img") 
        && (plain_str.starts_with("http://") || plain_str.starts_with("https://"));

                if is_single_image {
                    return Some(DragData::RemoteImage(vec![RemoteImage {
                        url: plain_str
                    }]));
                }

                return Some(DragData::RichText(RichContent {
                    html: html_str,
                    plain_text_fallback: plain_str,
                }));
            }

            // 处理纯文本类型数据
            let utf8_type = NSString::alloc(nil).init_str("public.utf8-plain-text");
            let text: *mut Object = msg_send![pasteboard, stringForType: utf8_type];
            if !text.is_null() {
                let utf8_ptr: *const c_char = msg_send![text, UTF8String];
                let text_str = CStr::from_ptr(utf8_ptr).to_str().unwrap_or("").to_owned();
                return Some(DragData::PlainText(text_str));
            }

            None
        })
    }
}
