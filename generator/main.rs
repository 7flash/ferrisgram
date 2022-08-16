
mod spec_types;
mod types;
mod methods;
mod helpers;
mod common;

// const SCHEMA_URL: &str = "https://raw.githubusercontent.com/PaulSonOfLars/telegram-bot-api-spec/main/api.json";

#[tokio::main]
pub async fn main() {
    match serde_json::from_str::<spec_types::ApiDescription>(common::read_file(String::from("api.json")).as_str()) {
        Ok(res) => {
            // types::generate_types(&res).await;
            methods::generate_methods(&res).await;
            // helpers::generate_helpers(&res).await;
        },
        Err(err) => println!("failed to parse json: {:?}", err),
    }   
}