use home::home_dir;
use openweathercli::api::request::{get_api_response, get_json};
use openweathercli::api::uri::construct_uri;
use openweathercli::config::config::get_config;

#[tokio::main]
async fn main() {
    // Get path to configuration directory
    // Surely there is a better way to handle this
    let confdir = match home_dir() {
        Some(path) => {
            let mut path = path.into_os_string()
                .into_string()
                .unwrap();
            path.push_str("/.config/owcli/config.yaml");
            path
        },
        None => "".to_string(),
    };

    let config = get_config(confdir);
    let uri = construct_uri(&config);
    let response = get_api_response(uri).await.expect("Failed to get API response!");
    let data = get_json(response).await.expect("Failed to parse json!");

    // Complete actions specified via command line
    config.actions(&data);
}
