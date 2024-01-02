use std::collections::HashMap;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread::{self, sleep};
use std::time::{Instant, Duration};
use std::task::Poll;

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
#[tauri::command]
pub async fn check_for_cards(deck: String, cards_with: Option<String>, in_field: String) -> std::result::Result<Vec<i64>, String> {

    match find_notes(
        &Client::new(),
        &deck,
        Some(&in_field),
        match cards_with {
            Some(e) => e,
            None => "".to_string(),
        },
    )
    .await {
        Ok(e) => Ok(e),
        Err(_) => Err("No cards found!".to_string())
    }
}
#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn edit_cards(
    cards: Vec<i64>,
    in_field: String,
    replace_with: String,
    findreplace: bool,
    find: String,
    del_newline: bool,
    as_space: Option<bool>,
) -> String {
    let client = Client::new();

    let counter = Arc::new(AtomicI32::from(0));

    let count = counter.clone();

    let thread1 = tokio::task::spawn(async move {
            if findreplace {
                find_and_replace(
                    &client,
                    &find,
                    &replace_with,
                    &in_field,
                    cards,
                    del_newline,
                    as_space,
                    count,
                )
                .await
                .unwrap();
            } else {
                replace_whole_fields(
                    &client,
                    cards,
                    &in_field,
                    &replace_with,
                    del_newline,
                    as_space,
                )
                .await
                .unwrap()
            }
    });

    let count = counter.clone();
    let poll_thread = tokio::task::spawn(async move {
        loop {
            println!("Value: {:?}", poll_count(count.clone()).await);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

    });
    thread1.await.unwrap();

    "Done!".to_string()
}
pub async fn get_models_from_deck(
    client: &Client,
    deck: &str,
) -> std::result::Result<Vec<String>, Error> {
    let notes = find_notes(client, deck, None, "*".to_string()).await.unwrap();
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
) -> std::result::Result<Vec<i64>, Error> {
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
    Ok(cards)
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
pub async fn get_models(client: &Client) -> Vec<Model> {
    let request: Request = Request {
        action: "modelNames".to_string(),
        version: 6,
        ..Default::default()
    };
    let model_names = get_req(ReqType::Models, client, request)
        .await
        .unwrap()
        .to_model_names();

    get_model_fields(client, model_names).await
}
pub async fn get_model_fields(client: &Client, models: Vec<String>) -> Vec<Model> {
    let mut models2: Vec<Model> = Vec::new();
    for model in models {
        let request: Request = Request {
            action: "modelFieldNames".to_string(),
            version: 6,
            params: Some(Params {
                model_name: Some(model.clone()),
                ..Default::default()
            }),
        };

        let fields = get_req(ReqType::ModelFields, client, request)
            .await
            .unwrap()
            .to_model_fields();
        models2.push(Model {
            name: model,
            fields,
        })
    }

    dbg!(&models2);
    models2
}
pub async fn poll_count(count: Arc<AtomicI32>) -> i32 {
    count.load(Ordering::Acquire)
}

// #[tokio::test]
// async fn polltest() {
//     let arc = Arc::new(AtomicI32);

//     let clone = arc.clone();
//     let a = tokio::task::spawn(async move {
//         let mut i = 0;
//         loop {
//             i += 1;
//             // *clone.lock().unwrap() = i;
//             tokio::time::sleep(Duration::from_millis(5)).await;
//         }
//     });

//     let clone = arc.clone();
//     let b = tokio::task::spawn(async move {
//         loop {
//             println!("Value: {:?}", poll_count(clone.clone()).await);
//             tokio::time::sleep(Duration::from_millis(10)).await;
//         }
//     });

//     a.await.unwrap();
//     b.await.unwrap();
// }
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

#[tokio::test]
async fn modelstest2() {
    let now = Instant::now();
    let _ = get_models(&Client::new()).await;
    println!("{:?} seconds elapsed", now.elapsed().as_secs());
}

