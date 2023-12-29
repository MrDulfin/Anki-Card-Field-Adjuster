use serde::{Serialize, Deserialize};


pub trait Response {
    fn get_response<T: Response>(self) -> PostResult;
}
pub trait Wrapper {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardResponse {
    pub result: Option<i64>,
    pub error: Option<String>
}
impl Response for CardResponse {
    fn get_response<T: Response>(self) -> PostResult {
        match self.result {
            Some(bunny) => PostResult::Card(bunny),
            None => PostResult::None
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardsResponse {
    pub result: Option<Vec<i64>>,
    pub error: Option<String>
}
impl Response for CardsResponse {
    fn get_response<T: Response>(self) -> PostResult {
        match self.result {
            Some(bunny) => PostResult::Cards(bunny),
            None => PostResult::None
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckResponse {
    pub result: Option<String>,
    pub error: Option<String>
}
impl Response for DeckResponse {
    fn get_response<T: Response>(self) -> PostResult {
        match self.result {
            Some(bunny) => PostResult::Deck(bunny),
            None => PostResult::None
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecksResponse {
    pub result: Option<Vec<String>>,
    pub error: Option<String>
}
impl Response for DecksResponse {
    fn get_response<T: Response>(self) -> PostResult {
        match self.result {
            Some(bunny) => PostResult::Decks(bunny),
            None => PostResult::None
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub result: Option<String>,
    pub error: Option<String>
}
impl Response for ModelResponse {
    fn get_response<T: Response>(self) -> PostResult {
        match self.result {
            Some(bunny) => PostResult::Model(bunny),
            None => PostResult::None
        }
    }

}
pub struct ModelsResponse {
    pub result: Option<Vec<String>>,
    pub error: Option<String>
}
impl Response for ModelsResponse {
    fn get_response<T: Response>(self) -> PostResult {
        match self.result {
            Some(bunny) => PostResult::Models(bunny),
            None => PostResult::None
        }
    }

}


#[derive(Debug)]
pub enum PostResult {
    Cards(Vec<i64>),
    Card(i64),
    Deck(String),
    Decks(Vec<String>),
    Model(String),
    Models(Vec<String>),
    None,
}

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
            _ => panic!("these aren't cards!")
        }
    }
    pub fn to_decks(self) -> Vec<String> {
        match self {
            PostResult::Decks(a) => a,
            _ => panic!("these aren't decks!")
        }
    }
    pub fn to_card(self) -> i64 {
        match self {
            PostResult::Card(a) => a,
            _ => panic!("this isn't a card!")
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
