use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CardResponse {
    pub result: Option<i64>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardsResponse {
    pub result: Option<Vec<i64>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecksResponse {
    pub result: Option<Vec<String>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub result: Option<Vec<String>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelFieldsResponse {
    pub result: Option<Vec<String>>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NoteInfoResponse {
    pub result: Vec<serde_json::Map<String, serde_json::Value>>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub enum PostResult {
    Cards(Vec<i64>),
    Decks(Vec<String>),
    Models(Vec<String>),
    ModelFields(Vec<String>),
    NotesInfo(Vec<serde_json::Map<String, serde_json::Value>>),
    None,
}
#[allow(dead_code)]
impl PostResult {
    pub fn to_cards(&self) -> Vec<i64> {
        match &self {
            PostResult::Cards(a) => a.to_owned(),
            _ => panic!("these aren't cards!"),
        }
    }
    pub fn to_decks(&self) -> Vec<String> {
        match &self {
            PostResult::Decks(a) => a.to_owned(),
            _ => panic!("these aren't decks!"),
        }
    }
    pub fn to_notes_info(&self) -> Vec<serde_json::Map<String, serde_json::Value>> {
        match &self {
            PostResult::NotesInfo(a) => a.to_owned(),
            _ => panic!("These aren't notes!"),
        }
    }
    pub fn to_model_names(&self) -> Vec<String> {
        match &self {
            PostResult::Models(a) => a.to_owned(),
            _ => panic!("Those aren't Model Names!"),
        }
    }
    pub fn to_model_fields(&self) -> Vec<String> {
        match &self {
            PostResult::ModelFields(a) => a.to_owned(),
            _ => panic!("Those aren't Model Fields!"),
        }
    }
}
