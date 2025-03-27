#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]

use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType};
use core_graphics::geometry::CGPoint;
use std::os::raw::c_void;
use std::sync::Mutex;
use cocoa::base::id;
use crate::LazyLock;

// 定义回调类型
pub type MouseEventCallback = fn(CGEventType, CGPoint);

// 事件处理回调函数
static CURRENT_MOUSE_LOCATION: LazyLock<Mutex<CGPoint>> = LazyLock::new(|| Mutex::new(CGPoint { x: 0.0, y: 0.0 }));

// 在事件回调中添加坐标更新
unsafe extern "C" fn event_callback(
    _proxy: id,
    event_type: CGEventType,
    cg_event: CGEvent,
    user_info: *mut c_void,
) -> CGEvent {
    // 在 unsafe 块中安全地解引用原始指针
    let callback = unsafe {
        // 检查指针是否为空
        if user_info.is_null() {
            panic!("回调函数指针为空");
        }
        &mut *(user_info as *mut MouseEventCallback)
    };
    let location = cg_event.location();
    callback(event_type, location);
    
    {
        let mut loc = CURRENT_MOUSE_LOCATION.lock().unwrap();
        loc.x = location.x;
        loc.y = location.y;
    }
    
    cg_event
}

// 新增公共接口函数
pub fn get_current_mouse_location() -> CGPoint {
    let loc = CURRENT_MOUSE_LOCATION.lock().unwrap();
    CGPoint { x: loc.x, y: loc.y }
}

// 开始监听鼠标事件
pub fn start_listening(callback: MouseEventCallback) {
    unsafe {
        let event_mask = (1 << CGEventType::LeftMouseDown as u64)
            | (1 << CGEventType::LeftMouseDragged as u64)
            | (1 << CGEventType::LeftMouseUp as u64);

        // 创建事件监听
        let tap = CGEventTapCreate(
            CGEventTapLocation::HID,
            0, // kCGHeadInsertEventTap
            1, // CGEventTapOption::ListenOnly
            event_mask,
            Some(event_callback),
            &callback as *const _ as *mut c_void,
        );

        if tap.is_null() {
            panic!("Failed to create event tap");
        }

        // 创建运行循环源
        let run_loop_source = CFMachPortCreateRunLoopSource(
            std::ptr::null_mut(),
            tap,
            0,
        );

        // 将运行循环源添加到当前运行循环
        CFRunLoopAddSource(
            CFRunLoopGetCurrent(),
            run_loop_source,
            kCFRunLoopCommonModes,
        );

        // 启用事件监听
        CGEventTapEnable(tap, true);

        // 运行运行循环
        CFRunLoopRun();
    }
}

// 引入 Core Graphics 和 Cocoa 框架的外部函数
#[link(name = "Cocoa", kind = "framework")]
unsafe extern "C" {
    pub fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: u32,
        options: u32,
        eventsOfInterest: u64,
        callback: Option<unsafe extern "C" fn(id, CGEventType, CGEvent, *mut c_void) -> CGEvent>,
        user_info: *mut c_void,
    ) -> *const c_void;
    pub fn CFMachPortCreateRunLoopSource(
        allocator: *const c_void,
        tap: *const c_void,
        order: u64,
    ) -> id;
    pub fn CFRunLoopAddSource(rl: id, source: id, mode: id);
    pub fn CFRunLoopGetCurrent() -> id;
    pub fn CGEventTapEnable(tap: *const c_void, enable: bool);
    pub fn CFRunLoopRun();
    pub static kCFRunLoopCommonModes: id;
}
