use dorazu_rs::{set_mouse_shake_callback, set_pasteboard_changed_callback, start_listening};

fn main() {
    // 注册拖拽粘贴板内容变化时的回调
    set_pasteboard_changed_callback(Box::new(|types| {
        println!("拖拽内容已变化! types: {:?}", types);
    }));

    // 注册鼠标摇动时的回调
    set_mouse_shake_callback(Box::new(|| {
        println!("检测到鼠标摇动!");
    }));

    // 开始监听鼠标事件
    start_listening();
}
