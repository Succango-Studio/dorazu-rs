use dorazu_rs::{set_mouse_shake_callback, set_pasteboard_changed_callback, start_listening};

fn main() {
    // 注册拖拽粘贴板内容变化时的回调
    set_pasteboard_changed_callback(Box::new(|data| {
        match data {
            Some(drag_data) => match drag_data {
                dorazu_rs::model::drag_types::DragData::LocalFile(paths) => {
                    println!("拖拽文件: {:?}", paths);
                }
                dorazu_rs::model::drag_types::DragData::PlainText(text) => {
                    println!("拖拽文本: {}", text);
                }
                dorazu_rs::model::drag_types::DragData::RichText(content) => {
                    println!("拖拽富文本: {:?}", content);
                }
                dorazu_rs::model::drag_types::DragData::RemoteImage(content) => {
                    println!("拖拽远程图片: {:?}", content);
                }
                _ => println!("其他类型拖拽数据"),
            },
            None => println!("无效的拖拽数据"),
        }
    }));

    // 注册鼠标摇动时的回调
    set_mouse_shake_callback(Box::new(|_data| {
        println!("检测到鼠标摇动!");
    }));

    // 开始监听鼠标事件
    start_listening();
}
