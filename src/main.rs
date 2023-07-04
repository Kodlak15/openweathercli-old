use openweathercli::utils::request::{get_api_response, get_json};
use openweathercli::utils::uri::construct_uri;

#[tokio::main]
async fn main() {
    let uri = construct_uri();
    let response = get_api_response(uri).await.unwrap();
    let json = get_json(response).await;

    // Example:
    println!("{}", json.wind.speed)
}
