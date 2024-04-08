use super::log::LogLevel;
use serde::{Deserialize, Serialize};
use std::{env, fs, path};

const CONFIG_PATH: &'static str = "/.config/dbv/";
const CONFIG_FILENAME: &'static str = "dbv.json";

type RGB = [u8; 3];

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "theme")]
    pub theme_config: ThemeConfig,

    #[serde(rename = "logLevel")]
    pub log_level: LogLevel,
}

impl Config {
    pub fn default() -> Self {
        Config {
            theme_config: ThemeConfig::default(),
            log_level: LogLevel::default(),
        }
    }
    pub fn load(&self) -> Self {
        let file_content = if let Ok(content) = fs::read_to_string(Config::get_path_config()) {
            content
        } else {
            return *self;
        };
        let value: Config = if let Ok(content) = serde_json::from_str(&file_content) {
            content
        } else {
            *self
        };
        value
    }

    pub fn init(&self) -> std::io::Result<()> {
        if path::Path::new(&Config::get_path_config()).exists() {
            return Ok(());
        }

        if !path::Path::new(&Config::get_path_config_folder()).exists() {
            fs::create_dir(Config::get_path_config_folder())?;
        }

        let content = serde_json::to_string_pretty(&Config::default())?;
        fs::write(Config::get_path_config(), content)?;

        Ok(())
    }

    fn get_path_config_folder() -> String {
        let home = env::var("HOME").expect("An error occured while reading $HOME.");
        format!("{}{}", home, CONFIG_PATH)
    }
    fn get_path_config() -> String {
        let home = env::var("HOME").expect("An error occured while reading $HOME.");
        format!("{}{}{}", home, CONFIG_PATH, CONFIG_FILENAME)
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
}

impl ThemeConfig {
    pub fn default() -> Self {
        let selected_color = [255, 165, 0];
        let unselected_color = [255, 236, 195];
        let help_text_color = [93, 169, 233];
        ThemeConfig {
            selected_color,
            unselected_color,
            help_text_color,
        }
    }
}
