use std::collections::HashMap;
// use std::error::Error;
use std::sync::Arc;

use fermi::use_read;
use reqwest::{Client, Error};
use serde::{ser, Deserialize, Serialize};

use crate::responses::{
    CardsResponse, DecksResponse, ModelFieldsResponse, ModelsResponse, NoteInfoResponse, PostResult,
};
use crate::{edits::*, CountState};

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
    pub fn fields(&self) -> &HashMap<String, String> {
        &self.fields
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Model {
    pub name: String,
    pub fields: Vec<String>,
}
pub async fn get_req(
    reqtype: ReqType,
    client: &Client,
    request: Request,
    server_port: (&str, &str),
) -> std::result::Result<PostResult, Error> {
    let s = server_port.0;
    let p = server_port.1;

    let res = client
        .post(format!("http://{s}:{p}/"))
        .json(&request)
        .send()
        .await?;

    match reqtype {
        ReqType::None => Ok(PostResult::None),
        ReqType::Cards => {
            let bun: CardsResponse = res.json().await?;
            Ok(PostResult::Cards(match bun.result {
                Some(e) => e,
                None => Vec::<i64>::new(),
            }))
        }
        ReqType::Decks => {
            let bun: DecksResponse = res.json().await.unwrap();
            Ok(PostResult::Decks(bun.result.unwrap()))
        }
        ReqType::ModelFields => {
            let bun: ModelFieldsResponse = res.json().await.unwrap();
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

pub async fn check_for_cards(
    deck: String,
    cards_with: String,
    in_field: String,
    server_port: (&str, &str),
) -> std::result::Result<Vec<i64>, String> {
    match find_notes(
        &Client::new(),
        &deck,
        Some(&in_field),
        cards_with,
        server_port,
    )
    .await
    {
        Ok(e) => Ok(e),
        Err(_) => Err("No cards found!".to_string()),
    }
}
#[allow(clippy::too_many_arguments)]
pub async fn edit_cards(
    cards: Vec<i64>,
    in_field: String,
    replace_with: String,
    findreplace: bool,
    find: String,
    del_newline: bool,
    as_space: Option<bool>,
    server_port: (&str, &str),
) -> Result<String, ()> {
    let client = Client::new();

    if findreplace {
        find_and_replace(
            &client,
            &find,
            &replace_with,
            &in_field,
            cards,
            del_newline,
            as_space,
            server_port,
        )
        .await
        .unwrap();
    } else {
        replace_whole_fields(&client, cards, &in_field, &replace_with, server_port)
            .await
            .unwrap()
    }

    Ok("Done!".to_string())
}
pub async fn deck_names(server_port: (&str, &str)) -> Vec<String> {
    let client = Client::new();

    let request: Request = Request {
        action: "deckNames".to_string(),
        version: 6,
        params: None,
    };
    get_req(ReqType::Decks, &client, request, server_port)
        .await
        .unwrap()
        .to_decks()
}

pub async fn find_notes(
    client: &Client,
    deck: &str,
    field: Option<&str>,
    cards_with: String,
    server_port: (&str, &str),
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
        let mut a = match get_req(ReqType::Cards, client, request, server_port).await {
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
                query: Some(format!(r#"deck:"{deck}" {bun}:*{cards_with}*"#).to_string()),
                ..Params::default()
            }),
        };

        let mut a = match get_req(ReqType::Cards, client, request, server_port).await {
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
        let mut a = match get_req(ReqType::Cards, client, request, server_port).await {
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
    server_port: (&str, &str),
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
        let bun = get_req(ReqType::NoteInfo, client, request, server_port)
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
                    //This turns the Map into a Hashmap for the fields
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

pub async fn get_models(server_port: (&str, &str)) -> Result<Vec<Model>, Error> {
    let client = &Client::new();
    let request: Request = Request {
        action: "modelNames".to_string(),
        version: 6,
        ..Default::default()
    };
    let model_names = get_req(ReqType::Models, client, request, server_port)
        .await
        .unwrap()
        .to_model_names();

    Ok(get_model_fields(client, model_names, server_port).await)
}
pub async fn get_model_fields(
    client: &Client,
    models: Vec<String>,
    server_port: (&str, &str),
) -> Vec<Model> {
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

        let fields = get_req(ReqType::ModelFields, client, request, server_port)
            .await
            .unwrap()
            .to_model_fields();
        models2.push(Model {
            name: model,
            fields,
        })
    }

    // dbg!(&models2);
    models2
}
