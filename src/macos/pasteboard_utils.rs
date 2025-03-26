#![allow(unexpected_cfgs)]
use std::ffi::CStr;
use std::os::raw::c_char;

use cocoa::base::nil;
use cocoa::foundation::NSString;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc::rc::autoreleasepool;

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

// 获取拖拽粘贴板中可用的内容类型
pub fn get_drag_pasteboard_types() -> Vec<String> {
    unsafe {
        autoreleasepool(|| {
            // 获取拖拽用的 NSPasteboard
            let cls = class!(NSPasteboard);
            let pasteboard: *mut Object = msg_send![cls, pasteboardWithName: NSPasteboardNameDrag];
            if pasteboard.is_null() {
                println!("Failed to get NSPasteboard!");
                return Vec::new();
            }

            // 先检查是否有文件拖拽数据，通过 "NSFilenamesPboardType"
            let filenames_ns = NSString::alloc(nil).init_str("NSFilenamesPboardType");
            let file_list: *mut Object = msg_send![pasteboard, propertyListForType: filenames_ns];
            if !file_list.is_null() {
                // file_list 应该为 NSArray
                let count: usize = msg_send![file_list, count];
                let mut results = Vec::with_capacity(count);
                for i in 0..count {
                    let file_ns: *mut Object = msg_send![file_list, objectAtIndex: i];
                    let utf8_ptr: *const c_char = msg_send![file_ns, UTF8String];
                    if !utf8_ptr.is_null() {
                        let file_str = CStr::from_ptr(utf8_ptr).to_str().unwrap_or("");
                        let path = std::path::Path::new(file_str);
                        if let Some(ext) = path.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                results.push(ext_str.to_owned());
                            } else {
                                results.push("unknown".to_owned());
                            }
                        } else {
                            results.push("unknown".to_owned());
                        }
                    }
                }
                return results;
            }

            // 如果不是文件，检查是否为纯文本拖拽，使用 "public.utf8-plain-text"
            let utf8_type = NSString::alloc(nil).init_str("public.utf8-plain-text");
            let text: *mut Object = msg_send![pasteboard, stringForType: utf8_type];
            if !text.is_null() {
                return vec!["text".to_owned()];
            }

            // 否则，返回unknown
            vec!["unknown".to_owned()]
        })
    }
}