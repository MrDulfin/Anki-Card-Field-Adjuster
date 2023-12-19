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
async fn post_req2(client: &Client, request: Request) -> Result<Vec<i64>, Error> {
    let res: Response2 = client.post("http://127.0.0.1:8765/")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;
    Ok(res.get_response())
    
}
async fn post_req3(client: &Client, request: Query) -> Result<(), Error> {
    client.post("http://127.0.0.1:8765/")
        .json(&request)
        .send()
        .await?;
    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
struct Query {
    action: String,
    version: i32,
    params:Params2
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
    note: Note
}
#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: i64,
    fields: HashMap<String, String>
}



pub async fn query_send(deck: String, cards_with: Option<String>, field: String, replace: String) -> String {
    dbg!(&cards_with);
    
    let client = Client::new();
    
    let cards = find_notes(&client, &deck, Some(&field), 
    match cards_with {
        Some(e) => e,
        None => "".to_string(),
    }).await;

    for card in cards {
        let mut field2: HashMap<String, String> = HashMap::with_capacity(1);
        field2.insert(field.clone(), replace.clone());

        let query: Query = Query { 
            action: "updateNoteFields".to_string(), 
            version: 6, 
            params: Params2{
                note: Note {
                    id: card,
                    fields: field2,
                }
            } 
        };

       _ = post_req3(&client, query).await;
    }
    "Done!".to_string()
}
#[derive(Serialize, Deserialize)]
struct  NoteInfo {
    #[serde(rename = "noteID")]
    note_id: i64,
    #[serde(rename = "modelName")]
    model_name: String,
    tags: Vec<String>,
    fields: HashMap<String, Vec<HashMap<String, String>>>,
}

pub async fn get_models(client: &Client, deck: &str) {
    let notes = find_notes(client, deck, None, "".to_string()).await;
    notes.iter().map(|n| {

    })
}

pub async fn find_notes(client: &Client, deck: &str, field: Option<&str>, cards_with: String) -> Vec<i64> {
    let mut cards = Vec::new();
    let bun = field.unwrap_or("You'll never see this");
    //get cards with field empty
    if field.is_some() && cards_with.is_empty() {
        let request: Request = Request { 
            action: "findNotes".to_string(), 
            version: 6, 
            params: Some(Params {  cards: None, query: Some(format!(r#"deck:"{deck}" {bun}:"#).to_string()) })
         };
        cards.append(&mut post_req2(client, request).await.unwrap());
    }else if field.is_some() && !cards_with.is_empty() {
        let request: Request = Request { 
            action: "findNotes".to_string(), 
            version: 6, 
            params: Some(Params {  cards: None, query: Some(format!(r#"deck:"{deck}" {bun}:{cards_with}"#).to_string()) })
         };
        cards.append(&mut post_req2(client, request).await.unwrap());
    }else if field.is_none() {
        let request: Request = Request { 
            action: "findNotes".to_string(), 
            version: 6, 
            params: Some(Params {  cards: None, query: Some(format!(r#"deck:"{deck}"#).to_string()) })
         };
        cards.append(&mut post_req2(client, request).await.unwrap());
    }
    cards
}