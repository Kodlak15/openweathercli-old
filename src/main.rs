use openweathercli::api::request::{get_api_response, get_json};
use openweathercli::api::uri::construct_uri;

#[tokio::main]
async fn main() {
    let uri = construct_uri();
    let response = get_api_response(uri).await.unwrap();
    let data = get_json(response).await.unwrap();

    // Example call:
    println!("{}", data.icon(true));
}
