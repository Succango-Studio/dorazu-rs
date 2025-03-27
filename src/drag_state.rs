use crate::shake_detector::ShakeDetector;
use crate::macos::pasteboard_utils::get_drag_pasteboard_change_count;

pub struct DragState {
    initial_change_count: i64,
    pasteboard_changed: bool,
    shake_detector: ShakeDetector,
    dragging_started: bool,
}

impl DragState {
    pub fn new() -> Self {
        Self {
            initial_change_count: 0,
            pasteboard_changed: false,
            shake_detector: ShakeDetector::new(),
            dragging_started: false,  // 新增初始化
        }
    }
    
    // 新增拖拽开始标记方法
    pub fn mark_dragging_started(&mut self) {
        self.dragging_started = true;
    }
    
    // 新增状态检查方法
    pub fn has_dragging(&self) -> bool {
        self.dragging_started
    }
    
    // 修改重置方法
    pub fn reset(&mut self) {
        self.initial_change_count = get_drag_pasteboard_change_count();
        self.pasteboard_changed = false;
        self.dragging_started = false;
        self.shake_detector.reset();
    }
    
    // 添加鼠标位置并检测摇动
    pub fn add_position(&mut self, x: f64, y: f64) {
        // 添加鼠标位置并检测摇动
        self.shake_detector.add_position(x, y);
    }
    
    // 检查粘贴板变化
    pub fn check_pasteboard_change(&mut self) -> bool {
        let current_count = get_drag_pasteboard_change_count();
        let pasteboard_changed = current_count != self.initial_change_count;
        
        if pasteboard_changed {
            // 标记粘贴板已变化，开始检测摇动
            self.pasteboard_changed = true;
            self.initial_change_count = current_count;
            
            // 重置摇动检测状态，从粘贴板变化后开始检测
            self.shake_detector.reset();
        }
        
        pasteboard_changed
    }
    
    // 检查是否处于摇动状态
    pub fn is_shaking(&self) -> bool {
        // 只有粘贴板变化后才检测摇动
        if !self.pasteboard_changed {
            return false;
        }
        
        self.shake_detector.is_shaking()
    }
    
    // 检查是否已在当前拖拽中检测到摇动
    pub fn is_shake_detected_in_current_drag(&self) -> bool {
        self.shake_detector.is_shake_detected_in_current_drag()
    }
    
    // 设置已在当前拖拽中检测到摇动
    pub fn set_shake_detected_in_current_drag(&mut self, value: bool) {
        self.shake_detector.set_shake_detected_in_current_drag(value);
    }
}