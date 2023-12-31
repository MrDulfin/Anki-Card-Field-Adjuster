use std::collections::HashMap;

use itertools::Itertools;
use reqwest::{Client, Error};

use crate::requests::{
    find_notes, get_req, notes_info, query_send, NoteInput, Params, ReqType, Request,
};
pub async fn replace_whole_fields(
    client: &Client,
    cards: Vec<i64>,
    field: &str,
    replace: &str,
) -> Result<(), Error> {
    for card in cards {
        let mut field2: HashMap<String, String> = HashMap::with_capacity(1);
        field2.insert(field.to_string(), replace.to_string());

        let request: Request = Request {
            action: "updateNoteFields".to_string(),
            version: 6,
            params: Some(Params {
                note: Some(NoteInput {
                    id: card,
                    fields: field2,
                }),
                ..Params::default()
            }),
        };
        _ = get_req(ReqType::None, client, request).await;
    }
    Ok(())
}
pub async fn find_and_replace(
    client: &Client,
    find: &str,
    replace_with: &str,
    in_field: &str,
    cards: Vec<i64>,
) -> Result<(), Error> {
    let notes_input: Vec<NoteInput> = cards
        .iter()
        .map(|note| NoteInput {
            id: *note,
            ..Default::default()
        })
        .collect();

    let replace: Vec<String> = notes_info(client, notes_input)
        .await
        .unwrap()
        .iter()
        .map(|note| {
            let a = note.fields().get_key_value(in_field).unwrap();
            a.1.replace(find, replace_with)
        })
        .collect();

    dbg!(&replace);

    for (i, card) in cards.into_iter().enumerate() {
        let mut field2: HashMap<String, String> = HashMap::with_capacity(1);
        field2.insert(in_field.to_string(), replace[i].clone());

        let request: Request = Request {
            action: "updateNoteFields".to_string(),
            version: 6,
            params: Some(Params {
                note: Some(NoteInput {
                    id: card,
                    fields: field2,
                }),
                ..Params::default()
            }),
        };
        _ = get_req(ReqType::None, client, request).await;
    }
    Ok(())
}
#[tokio::test]
async fn findreplace_test() {
    let cards = find_notes(&Client::new(), "Musical Notes", None, "*".to_string()).await;
    _ = find_and_replace(&Client::new(), "b", "a", "Front", cards).await;
}
