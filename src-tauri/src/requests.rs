use std::collections::HashMap;
use std::fmt::Result;
use std::ops::Deref;
use std::time::Instant;

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use itertools::Itertools;

use crate::responses::{
    CardResponse, CardsResponse, DeckResponse, DecksResponse, ModelResponse, ModelsResponse,
    NoteInfoResponse, PostResult, Response as BigRes,
};
use crate::{edits::*, get_decks, get_notes};

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
    //Get NoteInfo
    NoteInfo,
    //Get Nothing
    None,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Request {
    pub action: String,
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<NoteInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modelName")]
    pub model_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NoteInput {
    pub id: i64,
    pub fields: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NoteInfo {
    #[serde(alias = "noteID")]
    note_id: i64,
    #[serde(alias = "modelName")]
    model_name: String,
    tags: Vec<String>,
    fields: HashMap<String, String>,
}
impl NoteInfo {
    pub fn model(&self) -> &str {
        &self.model_name
    }
    pub fn fields(&self) -> &HashMap<String, String> {
        &self.fields
    }
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Model {
    name: String,
    fields: Vec<String>,
}
impl Model {
    pub fn from(name: String, fields: Vec<String>) -> Self {
        Model { name, fields }
    }
}

pub async fn get_req(
    reqtype: ReqType,
    client: &Client,
    request: Request,
) -> std::result::Result<PostResult, Error> {
    let res = client
        .post("http://127.0.0.1:8765/")
        .json(&request)
        .send()
        .await?;

    match reqtype {
        ReqType::None => Ok(PostResult::None),
        ReqType::Cards => {
            let bun: CardsResponse = res.json().await.unwrap();
            Ok(PostResult::Cards(bun.result.unwrap()))
        }
        ReqType::Decks => {
            let bun: DecksResponse = res.json().await.unwrap();
            Ok(PostResult::Decks(bun.result.unwrap()))
        }
        ReqType::ModelFields => {
            let bun: DecksResponse = res.json().await.unwrap();
            Ok(PostResult::ModelFields(bun.result.unwrap()))
        }
        ReqType::Models => {
            let bun: ModelsResponse = res.json().await.unwrap();
            Ok(PostResult::Models(bun.result.unwrap()))
        }
        ReqType::NoteInfo => {
            let bun: NoteInfoResponse = match res.json().await {
                Ok(e) => e,
                Err(e) => panic!("{e}"),
            };
            Ok(PostResult::NotesInfo(bun.result))
        }
        _ => Ok(PostResult::None),
    }
}

pub async fn query_send(
    deck: String,
    cards_with: Option<String>,
    field: String,
    replace: String,
    findreplace: bool,
) -> String {
    let client = Client::new();

    let cards = find_notes(
        &client,
        &deck,
        Some(&field),
        match cards_with {
            Some(e) => e,
            None => "".to_string(),
        },
    )
    .await;
    if findreplace {
        todo!()
    } else {
        replace_whole_fields(&client, cards, &field, &replace)
            .await
            .unwrap()
    }

    "Done!".to_string()
}
pub async fn get_models_from_deck(
    client: &Client,
    deck: &str,
) -> std::result::Result<Vec<String>, Error> {
    let notes = find_notes(client, deck, None, "*".to_string()).await;
    let notes_input: Vec<NoteInput> = notes
        .iter()
        .map(|note: &i64| NoteInput {
            id: *note,
            ..Default::default()
        })
        .collect();

    let model_names: Vec<String> = notes_info(client, notes_input)
        .await
        .unwrap()
        .iter()
        .map(|note| note.model_name.clone())
        .unique()
        .collect();
    Ok(model_names)
}
pub async fn deck_names() -> Vec<String> {
    let client = Client::new();

    let request: Request = Request {
        action: "deckNames".to_string(),
        version: 6,
        params: None,
    };
    get_req(ReqType::Decks, &client, request)
        .await
        .unwrap()
        .to_decks()
}

pub async fn find_notes(
    client: &Client,
    deck: &str,
    field: Option<&str>,
    cards_with: String,
) -> Vec<i64> {
    let mut cards = Vec::new();
    let bun = field.unwrap_or("You'll never see this");
    //get cards with field empty
    if field.is_some() && cards_with.is_empty() {
        let request: Request = Request {
            action: "findNotes".to_string(),
            version: 6,
            params: Some(Params {
                cards: None,
                query: Some(format!(r#"deck:"{deck}" {bun}:"#).to_string()),
                ..Params::default()
            }),
        };
        let mut a = match get_req(ReqType::Cards, client, request).await {
            Ok(e) => e.to_cards(),
            Err(e) => panic!("{e}"),
        };
        cards.append(&mut a);
    //get cards with something in the field
    } else if field.is_some() && !cards_with.is_empty() {
        let request: Request = Request {
            action: "findNotes".to_string(),
            version: 6,
            params: Some(Params {
                cards: None,
                query: Some(format!(r#"deck:"{deck}" {bun}:{cards_with}"#).to_string()),
                ..Params::default()
            }),
        };

        let mut a = match get_req(ReqType::Cards, client, request).await {
            Ok(e) => e.to_cards(),
            Err(e) => panic!("{e}"),
        };
        cards.append(&mut a);
    //get all cards from a deck
    } else if field.is_none() {
        let request: Request = Request {
            action: "findNotes".to_string(),
            version: 6,
            params: Some(Params {
                cards: None,
                query: Some(format!(r#"deck:"{deck}""#).to_string()),
                ..Params::default()
            }),
        };
        let mut a = match get_req(ReqType::Cards, client, request).await {
            Ok(e) => e.to_cards(),
            Err(e) => panic!("{e}"),
        };
        cards.append(&mut a);
    }
    cards
}
pub async fn notes_info(
    client: &Client,
    notes: Vec<NoteInput>,
) -> std::result::Result<Vec<NoteInfo>, Error> {
    let mut notes2: Vec<NoteInfo> = Vec::new();

    for note in notes {
        let request: Request = Request {
            action: "notesInfo".to_string(),
            version: 6,
            params: Some(Params {
                notes: Some(vec![note.id]),
                ..Default::default()
            }),
        };
        //Turn NoteInputs into NoteInfos
        let bun = get_req(ReqType::NoteInfo, client, request)
            .await
            .unwrap()
            .to_notes_info();

        for map in &bun {
            let mut hash: HashMap<String, String> = HashMap::new();
            map.get_key_value("fields")
                .unwrap()
                .1
                .as_object()
                .unwrap()
                .iter()
                .for_each(
                    //This mess turns the Map into a Hashmap for the fields
                    |(fieldname, value)| {
                        hash.insert(
                            fieldname.clone(),
                            value
                                .as_object()
                                .iter()
                                .map(|c| {
                                    c.get_key_value("value")
                                        .unwrap()
                                        .1
                                        .as_str()
                                        .unwrap()
                                        .to_string()
                                })
                                .collect::<Vec<String>>()
                                .first()
                                .unwrap()
                                .clone(),
                        );
                    },
                );
            notes2.push(NoteInfo {
                //For some reason grabbing the note ID directly didn't work for me
                note_id: map
                    .get_key_value("cards")
                    .unwrap()
                    .1
                    .as_array()
                    .unwrap()
                    .iter()
                    .last()
                    .unwrap()
                    .as_i64()
                    .unwrap(),
                model_name: map
                    .get_key_value("modelName")
                    .unwrap()
                    .1
                    .as_str()
                    .unwrap()
                    .to_string(),
                tags: map
                    .get_key_value("tags")
                    .unwrap()
                    .1
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|tag| tag.as_str().unwrap().to_string())
                    .collect(),
                fields: hash,
            })
        }
    }
    Ok(notes2)
}
pub async fn get_models() -> Vec<Model> {
    todo!()
}

#[tokio::test]
async fn multi_notes_info() {
    let now = Instant::now();
    let a = Client::new();
    let e = notes_info(
        &a,
        vec![
            NoteInput {
                id: 1703791651837,
                ..Default::default()
            },
            NoteInput {
                id: 1455649815706,
                ..Default::default()
            },
        ],
    )
    .await;
    println!("{:?}ms elapsed", now.elapsed().as_millis());
}
#[tokio::test]
async fn notes_info_() {
    let now = Instant::now();
    let a = Client::new();
    let e = notes_info(
        &a,
        vec![NoteInput {
            id: 1703791651837,
            ..Default::default()
        }],
    )
    .await;
    println!("{:?}ms elapsed", now.elapsed().as_millis());
}

#[tokio::test]
async fn modelstest() {
    let now = Instant::now();
    let _ = get_models_from_deck(&Client::new(), "JP Mining Note").await;
    println!("{:?} seconds elapsed", now.elapsed().as_secs());
}
