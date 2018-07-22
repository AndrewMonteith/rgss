extern crate reqwest;

use std;

fn try_download_http() -> reqwest::Result<String> {
    reqwest::get("https://anaminus.github.io/rbx/json/api/latest.json")?.text()
}

fn get_api_dir() -> std::path::PathBuf {
    let mut cur_path = std::env::current_dir().unwrap();
    cur_path.push("api.json");

    cur_path
}

pub fn get_api_string() -> Option<String> {
    match try_download_http() {
        Ok(json_text) => {
            save_api_to_file(&json_text);

            Some(json_text)
        }
        Err(_) => {
            println!("Failed to download latest api, will try and use local file");

            std::fs::read_to_string(get_api_dir()).ok()
        }
    }
}

fn save_api_to_file(text: &str) {
    std::fs::write(get_api_dir(), text).expect("failed to write file");
}