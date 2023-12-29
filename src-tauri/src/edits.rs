use std::collections::HashMap;

use reqwest::{Client, Error};

use crate::requests::{query_send, Query, Params, Note, find_notes, post_req, ReqType};
pub async fn replace_fields(client: &Client, deck: &str, cards_with: Option<String>, field: &str, replace: &str) -> Result<(), Error> {
    let cards = find_notes(client, deck, Some(field),
    match cards_with {
        Some(e) => e,
        None => "".to_string(),
    }).await;

    for card in cards {
        let mut field2: HashMap<String, String> = HashMap::with_capacity(1);
        field2.insert(field.to_string(), replace.to_string());

        let query: Query = Query {
            action: "updateNoteFields".to_string(),
            version: 6,
            params: Params{
                note: Some(Note {
                    id: card,
                    fields: field2,
                }),
                ..Params::default()
            }
        };

       _ = post_req(&client, query).await;
    }
    Ok(())
}