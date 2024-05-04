use std::env;

const APP_PATH: &'static str = "/.config/dbv/";

pub fn get_path_app_folder() -> String {
    let home = env::var("HOME").expect("An error occured while reading $HOME.");
    format!("{}{}", home, APP_PATH)
}
pub fn get_path_app_file(filename: &str) -> String {
    let home = env::var("HOME").expect("An error occured while reading $HOME.");
    format!("{}{}{}", home, APP_PATH, filename)
}
