use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    result: Option<Vec<String>>,
    error: Option<String>
}
impl Response {
    fn get_response(&self) -> Vec<String>{
       let sugar: Vec<String> = match &self.result {
        Some(bunny) => bunny.to_vec(),
        None => panic!()
       };
       sugar
    }

}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    action: String,
    version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Params>
}

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    cards: Vec<i64>
}

#[tauri::command]
pub async fn deck_names() -> Vec<String> {
    let client = Client::new();

    let request: Request = Request {
        action: "deckNames".to_string(),
        version: 6,
        params: None,
    };

    let result = post_req(client, request).await;

    result.unwrap()
}

async fn post_req(client: Client, request: Request) -> Result<Vec<String>, Error> {
    let res: Response = client.post("http://127.0.0.1:8765/")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;
    Ok(res.get_response())
}
