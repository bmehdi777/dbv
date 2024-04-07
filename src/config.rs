use serde::{Deserialize, Serialize};

type RGB = [u8; 3];

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub selected_color: RGB,
    pub unselected_color: RGB,
}

impl ThemeConfig {
    pub fn new() -> Self {
        let selected_color = [255, 165, 0];
        let unselected_color = [255, 236, 195];
        ThemeConfig {
            selected_color,
            unselected_color,
        }
    }
}
