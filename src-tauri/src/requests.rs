use std::collections::HashMap;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    cards: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>
}


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
async fn post_req2(client: Client, request: Request) -> Result<Vec<i64>, Error> {
    let res: Response2 = client.post("http://127.0.0.1:8765/")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;
    Ok(res.get_response())
    
}
#[derive(Debug, Serialize, Deserialize)]
struct Query {
    action: String,
    version: i32,
    params:Params
}
#[derive(Debug, Serialize, Deserialize)]
struct Response2 {
    result: Option<Vec<i64>>,
    error: Option<String>
}
impl Response2 {
    fn get_response(&self) -> Vec<i64>{
       let sugar: Vec<i64> = match &self.result {
        Some(bunny) => bunny.to_vec(),
        None => panic!()
       };
       sugar
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Params2 {
    id: i64,
    fields: HashMap<String, String>
}


pub async fn query_send(deck: String, cards_with: String, field: String, replace: String) {
    let client = Client::new();
    let request: Request = Request { 
        action: "findNotes".to_string(), 
        version: 6, 
        params: Some(Params {  cards: None, query: Some(format!("deck:{deck}").to_string()) })
     };
    let cards: Vec<i64> = post_req2(client, request).await.unwrap();

    for card in cards {
        let query: Query = Query { 
            action: "updateNoteFields".to_string(), 
            version: 6, 
            params: Params2{
                id: card,
                fields:
            } }
    }

}