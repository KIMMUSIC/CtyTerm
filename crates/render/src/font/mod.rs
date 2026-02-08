#[derive(Debug, Clone)]
pub struct FontConfig {
    pub family: String,
    pub size_px: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "Cascadia Mono".to_owned(),
            size_px: 14.0,
        }
    }
}
