#[derive(Debug, Clone)]
pub enum DragData {
    LocalFile(Vec<std::path::PathBuf>),
    RemoteImage(Vec<RemoteImage>),
    PlainText(String),
    RichText(RichContent),
}

#[derive(Debug, Clone)]
pub struct RemoteImage {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct RichContent {
    pub html: String,
    pub plain_text_fallback: String,
}
