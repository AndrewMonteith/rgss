mod downloader;
mod api;
mod parser;

pub use self::api::RobloxApi;

pub fn initalise() -> Result<RobloxApi, &'static str> {
    match downloader::get_api_string() {
        Some(text) => Ok(parser::load_api(&text)),
        None => Err("failed to find a copy of the roblox api")
    }
}
