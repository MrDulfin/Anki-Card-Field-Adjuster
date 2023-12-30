use std::collections::HashMap;
use std::fmt::Result;
use std::ops::Deref;

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{edits::*, get_decks, get_notes};
use crate::responses::{PostResult, CardResponse, CardsResponse, DeckResponse, DecksResponse, ModelResponse, ModelsResponse, Response as BigRes};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ReqType {
    //Get ??
    Deck,
    //Get Vec<String> for Decks
    Decks,
    //Get ??
    Card,
    //Get a Vec<i64> for each card
    Cards,
    //Get all fields for a model
    ModelFields,
    //Get Vec<String> for Models
    Models,
    //Get Nothing
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub action: String,
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Note>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub fields: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct NoteInfo {
    #[serde(rename = "noteID")]
    note_id: i64,
    #[serde(rename = "modelName")]
    model_name: String,
    tags: Vec<String>,
    fields: HashMap<String, Vec<HashMap<String, String>>>,
}

#[allow(unreachable_code)]
pub async fn get_req(reqtype: ReqType, client: &Client, request: Request) -> std::result::Result<PostResult, Error> {
    let res = client.post("http://127.0.0.1:8765/")
        .json(&request)
        .send()
        .await?;

    match reqtype {
        ReqType::None => {
            Ok(PostResult::None)
        }
        ReqType::Cards => {
            let bun: CardsResponse = res.json().await.unwrap();
            Ok(PostResult::Cards(bun.result.unwrap()))
        },
        ReqType::Decks => {
            let bun: DecksResponse = res.json().await.unwrap();
            Ok(PostResult::Decks(bun.result.unwrap()))
        }
        // ReqType::ModelFields => {
        //     let bun: DecksResponse = res.json().await.unwrap();
        //     Ok(PostResult::ModelFields(bun.result.unwrap()))
        // }
        ReqType::Models => {
            let bun: ModelsResponse = res.json().await.unwrap();
            Ok(PostResult::Models(bun.result.unwrap()))
        }
        _ => Ok(PostResult::None),
    }
}

pub async fn query_send(deck: String, cards_with: Option<String>, field: String, replace: String) -> String {

    let client = Client::new();

    let cards = find_notes(&client, &deck, Some(&field), match cards_with {
        Some(e) => e,
        None => "".to_string(),
    }).await;

    for card in cards {
        let mut field2: HashMap<String, String> = HashMap::with_capacity(1);
        field2.insert(field.clone(), replace.clone());

        let request: Request = Request {
            action: "updateNoteFields".to_string(),
            version: 6,
            params: Some(Params {
                note: Some(Note {
                    id: card,
                    fields: field2,
                }),
                ..Default::default()
            }),
        };

        let _ = get_req(ReqType::None, &client, request).await;
    }
    "Done!".to_string()
}

//TODO: get_models()
// pub async fn get_models(client: &Client, deck: &str) {
//     let notes = find_notes(client, deck, None, "".to_string()).await;
//     notes.iter().map(|n| {

//     })
// }
// pub async fn get_model(client: &Client, card:)
pub async fn deck_names() -> Vec<String> {
    let client = Client::new();

    let request: Request = Request {
        action: "deckNames".to_string(),
        version: 6,
        params: None,
    };
   get_req(ReqType::Decks, &client, request).await.unwrap().to_decks()
}

pub async fn find_notes(client: &Client, deck: &str, field: Option<&str>, cards_with: String) -> Vec<i64> {
    let mut cards = Vec::new();
    let bun = field.unwrap_or("You'll never see this");
    //get cards with field empty
    if field.is_some() && cards_with.is_empty() {
        let request: Request = Request {
            action: "findNotes".to_string(),
            version: 6,
            params: Some(Params {  cards: None, query: Some(format!(r#"deck:"{deck}" {bun}:"#).to_string()), ..Params::default() })
         };
         let mut a = match get_req(ReqType::Cards, client, request).await {
            Ok(e) => e.to_cards(),
            Err(e) => panic!("{e}"),
       };
       cards.append(&mut a);
    //get cards with something in the field
    }else if field.is_some() && !cards_with.is_empty() {
        let request: Request = Request {
            action: "findNotes".to_string(),
            version: 6,
            params: Some(Params {  cards: None, query: Some(format!(r#"deck:"{deck}" {bun}:{cards_with}"#).to_string()), ..Params::default() })
         };

           let mut a = match get_req(ReqType::Cards, client, request).await {
                Ok(e) => e.to_cards(),
                Err(e) => panic!("{e}"),
           };
           cards.append(&mut a);
    //get all cards from a deck
    }else if field.is_none() {
        let request: Request = Request {
            action: "findNotes".to_string(),
            version: 6,
            params: Some(Params {  cards: None, query: Some(format!(r#"deck:"{deck}"#).to_string()), ..Params::default()})
         };
         let mut a = match get_req(ReqType::Cards, client, request).await {
            Ok(e) => e.to_cards(),
            Err(e) => panic!("{e}"),
       };
       cards.append(&mut a);
    }
    cards
}

#[tokio::test]
async fn test_() {
    let e = deck_names().await;
    println!("{:?}", e);
}