use std::{collections::HashMap, time::Instant};

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
    del_newline: bool,
    as_space: Option<bool>
) -> Result<(), Error> {
    let notes_input: Vec<NoteInput> = cards
        .iter()
        .map(|note: &i64| NoteInput {
            id: *note,
            ..Default::default()
        })
        .collect();

    let mut replace: Vec<String> = notes_info(client, notes_input)
        .await
        .unwrap()
        .iter()
        .map(|note: &crate::requests::NoteInfo| {
            let a: (&String, &String) = note.fields().get_key_value(in_field).unwrap();
                a.1.replace(find, replace_with)
        })
        .collect();

        if del_newline {
            let mut ny = Vec::new();
            for r in &replace {
                let bun = remove_newlines(r, as_space.unwrap()).await;
                ny.push(bun);
            }
            replace.clear();
            replace.append(&mut ny);
        }

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
pub async fn remove_newlines(text: &str, as_space: bool) -> String {
    if as_space {
        text.replace("\n", " ")
    }else {
        text.replace("\n", "")
    }
}



#[tokio::test]
async fn findreplace_test() {
    let now = Instant::now();
    let cards = find_notes(&Client::new(), "JP Mining Note", None, "*".to_string()).await;
    _ = find_and_replace(&Client::new(), "\n", "", "Sentence", cards, false, Some(false)).await;
    println!("{:?} seconds", now.elapsed().as_secs());
}
