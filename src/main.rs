#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod rg;
mod roblox;

use rg::parser;

fn compile_str(contents: &str) {
    match parser::parse_str(contents) {
        Ok(api) => println!("{}", serde_json::to_string(&api).unwrap()),
        Err(error) => println!("Error when compiling {}", error),
    }
}

fn main() {
    let contents = r#"ScreenGui {
    Name: "WelcomeGui"
    Enabled: true

    TextLabel {
        Text: "Welcome"
        BackgroundColor3: #FF00FF
        TextStrokeTransparency: 0.2
        BorderSizePixel: 0
    }

    TextButton "Continue" {
        Text: "Continue"
        BackgroundTransparency: 1
        TextXAlignment: Center
        TextColor3: RGB(255, 20, 120)
    }
}"#;

    compile_str(contents);



}
