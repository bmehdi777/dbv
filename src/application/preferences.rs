use crate::{log::LogLevel, utils};
use serde::{Deserialize, Serialize};
use std::{fs, path};

const CONFIG_FILENAME: &'static str = "dbv.json";

type RGB = [u8; 3];

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Preference {
    #[serde(rename = "theme")]
    pub theme_config: ThemeConfig,

    #[serde(rename = "logLevel")]
    pub log_level: LogLevel,
}

impl Preference {
    pub fn default() -> Self {
        Preference {
            theme_config: ThemeConfig::default(),
            log_level: LogLevel::default(),
        }
    }
    pub fn load(&self) -> Self {
        let file_content = if let Ok(content) = fs::read_to_string(utils::get_path_app_file(CONFIG_FILENAME)) {
            content
        } else {
            return *self;
        };
        let value: Preference = if let Ok(content) = serde_json::from_str(&file_content) {
            content
        } else {
            *self
        };
        value
    }

    pub fn init(&self) -> std::io::Result<()> {
        if path::Path::new(&utils::get_path_app_file(CONFIG_FILENAME)).exists() {
            return Ok(());
        }

        if !path::Path::new(&utils::get_path_app_folder()).exists() {
            fs::create_dir(utils::get_path_app_folder())?;
        }

        let content = serde_json::to_string_pretty(&Preference::default())?;
        fs::write(utils::get_path_app_file(CONFIG_FILENAME), content)?;

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(rename = "selectedColor")]
    pub selected_color: RGB,
    #[serde(rename = "unselectedColor")]
    pub unselected_color: RGB,
    #[serde(rename = "helpTextColor")]
    pub help_text_color: RGB,

    #[serde(rename = "helpKeyColor")]
    pub help_key_color: RGB,
    #[serde(rename = "helpDescColor")]
    pub help_desc_color: RGB
}

impl ThemeConfig {
    pub fn default() -> Self {
        let selected_color = [255, 165, 0];
        let unselected_color = [255, 236, 195];
        let help_text_color = [93, 169, 233];
        let help_key_color = [100,149,237];
        let help_desc_color = [255, 236, 195];
        ThemeConfig {
            selected_color,
            unselected_color,
            help_text_color,
            help_key_color,
            help_desc_color,
        }
    }
}
