use std::sync::Mutex;
use core_graphics::event::CGEventType;
use core_graphics::geometry::CGPoint;
use macos::pasteboard_utils::get_drag_pasteboard_types;

mod shake_detector;
mod drag_state;
mod macos;

use crate::macos::mouse_listener;
use drag_state::DragState;
use std::sync::LazyLock;

/// 当拖拽粘贴板内容变化时的回调类型，参数为粘贴板类型列表
pub type PasteboardChangedCallback = Box<dyn Fn(Vec<String>) + Send + Sync + 'static>;
/// 当检测到鼠标摇动时的回调类型
pub type MouseShakeCallback = Box<dyn Fn() + Send + Sync + 'static>;

/// 内部状态，保存拖拽过程中的数据
static STATE: LazyLock<Mutex<DragState>> = LazyLock::new(|| Mutex::new(DragState::new()));
/// 拖拽粘贴板变化回调
static PASTEBOARD_CALLBACK: LazyLock<Mutex<Option<PasteboardChangedCallback>>> =
    LazyLock::new(|| Mutex::new(None));
/// 鼠标摇动回调
static SHAKE_CALLBACK: LazyLock<Mutex<Option<MouseShakeCallback>>> =
    LazyLock::new(|| Mutex::new(None));

/// 设置拖拽粘贴板内容变化的回调函数
pub fn set_pasteboard_changed_callback(cb: PasteboardChangedCallback) {
    let mut callback = PASTEBOARD_CALLBACK.lock().unwrap();
    *callback = Some(cb);
}

/// 设置鼠标摇动检测的回调函数
pub fn set_mouse_shake_callback(cb: MouseShakeCallback) {
    let mut callback = SHAKE_CALLBACK.lock().unwrap();
    *callback = Some(cb);
}

/// 内部回调，由鼠标监听模块调用，根据事件类型和位置进行处理
fn callback(event_type: CGEventType, location: CGPoint) {
    let mut state = STATE.lock().unwrap();

    match event_type {
        CGEventType::LeftMouseDown => {
            // 重置状态
            state.reset();
        }
        CGEventType::LeftMouseDragged => {
            let x = location.x;
            let y = location.y;
            // 添加鼠标位置，检测摇动
            state.add_position(x, y);

            // 检查拖拽时粘贴板是否有变化
            if state.check_pasteboard_change() {
                let types = get_drag_pasteboard_types();
                if let Some(ref cb) = *PASTEBOARD_CALLBACK.lock().unwrap() {
                    cb(types);
                }
            }

            // 检查是否检测到摇动且当前拖拽中还未处理
            if state.is_shaking() && !state.is_shake_detected_in_current_drag() {
                if let Some(ref cb) = *SHAKE_CALLBACK.lock().unwrap() {
                    cb();
                }
                state.set_shake_detected_in_current_drag(true);
            }
        }
        _ => {}
    }
}

/// 开始监听鼠标事件
pub fn start_listening() {
    mouse_listener::start_listening(callback);
}
