use reqwest::{Response, Error};
use crate::utils::data::Data;

pub async fn get_api_response(uri: String) -> Result<Response, Error> {
    reqwest::get(uri).await
}

pub async fn get_json(response: Response) -> Data {
    response.json::<Data>().await.unwrap()
}
