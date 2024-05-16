mod api;

use api::api::make_api_request;

use crate::api::api::LoremIpsumParams;

#[tokio::main]
async fn main() {
    println!("Running ipsum wizard");

    let args: Vec<String> = std::env::args().collect();

    // ||
    println!("Program name: {}", args[0]);

    if args.len() > 0 {
        let character_length: String;
        if !args.len() > 1 || args[1].is_empty() {
            character_length = String::from("500");
        } else {
            character_length = args[1].to_owned();
        }

        println!("Characters: {}", character_length);

        let paragraphs: String;
        if !args.len() > 2 || args[2].is_empty() {
            paragraphs = String::from("1");
        } else {
            paragraphs = args[2].to_owned();
        }

        println!("Paragraphs: {}", paragraphs);

        let params: LoremIpsumParams = LoremIpsumParams {
            max_length: Some(character_length),
            paragraphs: Some(paragraphs),
            start_with_lorem_ipsum: Some(true),
            random: Some(true),
        };

        make_api_request(params).await;
    } else {
        println!("No starting parameters provided.");
    }
}
