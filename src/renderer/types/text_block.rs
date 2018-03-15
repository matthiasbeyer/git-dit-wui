#[derive(Serialize)]
pub struct TextBlock {
    text: String
}

impl TextBlock {
    pub fn new(s: String) -> TextBlock {
        TextBlock { text: s }
    }
}

