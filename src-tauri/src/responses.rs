use serde::{Deserialize, Serialize};

use crate::requests::NoteInfo;

// pub trait Response2<T>: Response {

// }
pub trait Response {}
pub trait Wrapper {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardResponse {
    pub result: Option<i64>,
    pub error: Option<String>,
}
impl Response for CardResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardsResponse {
    pub result: Option<Vec<i64>>,
    pub error: Option<String>,
}
impl Response for CardsResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckResponse {
    pub result: Option<String>,
    pub error: Option<String>,
}
impl Response for DeckResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecksResponse {
    pub result: Option<Vec<String>>,
    pub error: Option<String>,
}
impl Response for DecksResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub result: Option<String>,
    pub error: Option<String>,
}
impl Response for ModelResponse {}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub result: Option<Vec<String>>,
    pub error: Option<String>,
}
impl Response for ModelsResponse {}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelFieldsResponse {
    pub result: Option<Vec<String>>,
    pub error: Option<String>,
}
impl Response for ModelFieldsResponse {}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NoteInfoResponse {
    pub result: Vec<serde_json::Map<String, serde_json::Value>>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub enum PostResult {
    Cards(Vec<i64>),
    Card(i64),
    Deck(String),
    Decks(Vec<String>),
    Model(String),
    Models(Vec<String>),
    ModelFields(Vec<String>),
    NotesInfo(Vec<serde_json::Map<String, serde_json::Value>>),
    NoteInfo(NoteInfoResponse),
    None,
}
#[allow(dead_code)]
impl PostResult {
    pub fn is_none(&self) -> bool {
        matches!(self, PostResult::None)
    }
    pub fn is_cards(&self) -> bool {
        matches!(self, PostResult::Cards(_))
    }
    pub fn is_decks(&self) -> bool {
        matches!(self, PostResult::Decks(_))
    }
    pub fn to_cards(self) -> Vec<i64> {
        match self {
            PostResult::Cards(a) => a,
            _ => panic!("these aren't cards!"),
        }
    }
    pub fn to_decks(self) -> Vec<String> {
        match self {
            PostResult::Decks(a) => a,
            _ => panic!("these aren't decks!"),
        }
    }
    pub fn to_card(self) -> i64 {
        match self {
            PostResult::Card(a) => a,
            _ => panic!("this isn't a card!"),
        }
    }
    pub fn to_notes_info(self) -> Vec<serde_json::Map<String, serde_json::Value>> {
        match self {
            PostResult::NotesInfo(a) => a,
            _ => panic!("These aren't notes!"),
        }
    }
    pub fn to_model_names(self) -> Vec<String> {
        match self {
            PostResult::Models(a) => a,
            _ => panic!("Those aren't Model Names!"),
        }
    }
    pub fn to_model_fields(self) -> Vec<String> {
        match self {
            PostResult::ModelFields(a) => a,
            _ => panic!("Those aren't Model Fields!"),
        }
    }
}
impl From<Vec<i64>> for PostResult {
    fn from(value: Vec<i64>) -> Self {
        PostResult::Cards(value)
    }
}
impl From<i64> for PostResult {
    fn from(value: i64) -> Self {
        PostResult::Card(value)
    }
}
impl From<Vec<String>> for PostResult {
    fn from(value: Vec<String>) -> Self {
        PostResult::Decks(value)
    }
}
impl From<String> for PostResult {
    fn from(value: String) -> Self {
        PostResult::Deck(value)
    }
}
