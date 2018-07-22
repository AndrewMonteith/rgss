mod rg;
mod roblox;

use rg::parser;

fn compile_str(contents: &str) -> Result<(), parser::ParseError> {
    let tree = parser::parse_str(contents)?;

    Ok(())
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
        TextColor3: RGB(255, 20, 120)
    }
}"#;

    match roblox::initalise() {
        Ok(api) => println!("{:?}", compile_str(contents)),
        Err(error_msg) => eprintln!("Error when loading roblox api {}", error_msg)
    }



}
