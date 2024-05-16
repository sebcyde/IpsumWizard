pub mod api {
    use reqwest::{header, Client, Response};
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct LoremIpsumParams {
        pub max_length: Option<String>,
        pub paragraphs: Option<String>,
        pub start_with_lorem_ipsum: Option<bool>,
        pub random: Option<bool>,
    }

    pub async fn make_api_request(params: LoremIpsumParams) {
        let url: &str = "https://api.api-ninjas.com/v1/loremipsum";
        let client: Client = Client::new();
        let mut endpoint: String;
        let _auth_token: &str;

        if params.max_length.is_some() {
            endpoint = format!("{}?paragraphs={}", url, params.paragraphs.unwrap());
        } else {
            endpoint = format!("{}?paragraphs={}", url, params.paragraphs.unwrap());
        }

        if params.max_length.is_some() {
            endpoint = format!("{}?max_length={}", endpoint, params.max_length.unwrap());
        }

        let response: Response = client
            .get(endpoint)
            // .header(header::AUTHORIZATION, auth_token)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .expect("Failed to send request.");

        if response.status().is_success() {
            let data: String = response.text().await.unwrap();
            println!("Data: {:?}", data);
        } else {
            println!("Error: {:?}", response);
        }
    }
}
