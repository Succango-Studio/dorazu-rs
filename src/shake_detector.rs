use std::time::{Instant, Duration};
use std::collections::VecDeque;

// 鼠标位置记录，包含位置和时间戳
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
    pub timestamp: Instant,
}

pub struct ShakeDetector {
    // 存储最近的鼠标位置
    position_history: VecDeque<MousePosition>,
    // 上一次方向变化的时间
    last_direction_change: Option<Instant>,
    // 方向变化计数
    direction_changes: usize,
    // 上一次检测到的方向 (x方向: true为右, false为左)
    last_x_direction: Option<bool>,
    // 上一次检测到的方向 (y方向: true为下, false为上)
    last_y_direction: Option<bool>,
    // 是否处于摇动状态
    is_shaking: bool,
    // 是否已经在当前拖拽中检测到摇动并处理
    shake_detected_in_current_drag: bool,
}

impl ShakeDetector {
    pub fn new() -> Self {
        Self {
            position_history: VecDeque::with_capacity(10),
            last_direction_change: None,
            direction_changes: 0,
            last_x_direction: None,
            last_y_direction: None,
            is_shaking: false,
            shake_detected_in_current_drag: false,
        }
    }
    
    // 添加鼠标位置到历史记录
    pub fn add_position(&mut self, x: f64, y: f64) {
        let now = Instant::now();
        
        // 添加新位置
        self.position_history.push_back(MousePosition {
            x,
            y,
            timestamp: now,
        });
        
        // 移除过旧的记录，只保留最近500ms内的记录
        while let Some(pos) = self.position_history.front() {
            if now.duration_since(pos.timestamp) > Duration::from_millis(500) {
                self.position_history.pop_front();
            } else {
                break;
            }
        }
        
        // 检测方向变化
        self.detect_direction_change();
        
        // 检测是否为摇动
        self.detect_shake();
    }
    
    // 检测方向变化
    fn detect_direction_change(&mut self) {
        if self.position_history.len() < 3 {
            return;
        }
        
        // 获取最近的两个位置
        let positions: Vec<&MousePosition> = self.position_history.iter().rev().take(3).collect();
        let current = positions[0];
        let previous = positions[1];
        let before_previous = positions[2];
        
        // 计算当前移动方向
        let current_x_direction = current.x > previous.x;
        let current_y_direction = current.y > previous.y;
        
        // 计算前一次移动方向
        let previous_x_direction = previous.x > before_previous.x;
        let previous_y_direction = previous.y > before_previous.y;
        
        // 检测X或Y方向是否发生变化
        let x_direction_changed = current_x_direction != previous_x_direction;
        let y_direction_changed = current_y_direction != previous_y_direction;
        
        // 如果方向发生变化
        if x_direction_changed || y_direction_changed {
            let now = Instant::now();
            
            // 更新方向变化时间
            if let Some(last_change) = self.last_direction_change {
                // 如果方向变化间隔小于200ms，增加方向变化计数
                if now.duration_since(last_change) < Duration::from_millis(200) {
                    self.direction_changes += 1;
                } else {
                    // 重置计数
                    self.direction_changes = 1;
                }
            }
            
            self.last_direction_change = Some(now);
            self.last_x_direction = Some(current_x_direction);
            self.last_y_direction = Some(current_y_direction);
        }
    }
    
    // 检测是否为摇动
    fn detect_shake(&mut self) {
        // 如果在短时间内方向变化次数超过阈值，判定为摇动
        if self.direction_changes >= 4 {
            self.is_shaking = true;
        } else {
            self.is_shaking = false;
        }
    }
    
    // 重置摇动检测状态
    pub fn reset(&mut self) {
        self.direction_changes = 0;
        self.is_shaking = false;
        self.position_history.clear();
        self.last_direction_change = None;
        self.last_x_direction = None;
        self.last_y_direction = None;
        self.shake_detected_in_current_drag = false;
    }
    
    // 获取当前是否处于摇动状态
    pub fn is_shaking(&self) -> bool {
        self.is_shaking
    }
    
    // 获取当前是否已在当前拖拽中检测到摇动
    pub fn is_shake_detected_in_current_drag(&self) -> bool {
        self.shake_detected_in_current_drag
    }
    
    // 设置已在当前拖拽中检测到摇动
    pub fn set_shake_detected_in_current_drag(&mut self, value: bool) {
        self.shake_detected_in_current_drag = value;
    }
}