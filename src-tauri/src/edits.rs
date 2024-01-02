use std::{collections::HashMap, time::{Instant, Duration}, sync::{Arc, Mutex, atomic::{AtomicI32, Ordering}}};
use core::future::poll_fn;
use std::task::{Context, Poll};

use itertools::Itertools;
use reqwest::{Client, Error};

use crate::requests::{
    find_notes, get_req, notes_info, edit_cards, NoteInput, Params, ReqType, Request, poll_count,
};
pub async fn replace_whole_fields(
    client: &Client,
    cards: Vec<i64>,
    field: &str,
    replace: &str,
    del_newline: bool,
    as_space: Option<bool>,
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
#[allow(clippy::too_many_arguments)]
pub async fn find_and_replace(
    client: &Client,
    find: &str,
    replace_with: &str,
    in_field: &str,
    cards: Vec<i64>,
    del_newline: bool,
    as_space: Option<bool>,
    count: Arc<AtomicI32>,
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
            let field: String = note
                .fields()
                .get_key_value(in_field)
                .unwrap()
                .1
                .replace(find, replace_with);

            if del_newline {
                remove_newlines(&field, as_space.unwrap())
            } else {
                field
            }
        })
        .collect();

    dbg!(&replace);

    println!("for loop here");
    let mut processed_cards = 0;

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

        processed_cards += 1;
        count.store(processed_cards as i32, Ordering::Release);
    }
    Ok(())
}
pub fn remove_newlines(text: &str, as_space: bool) -> String {
    if as_space {
        text.replace("<br>", " ")
    } else {
        text.replace("<br>", "")
    }
}

#[tokio::test]
async fn findreplace_test() {

    let arc = Arc::new(AtomicI32::from(0));

    let clone = arc.clone();
    let a = tokio::task::spawn(async move {
        let now = Instant::now();
        let cards = find_notes(&Client::new(), "TheMoeWay Tango N5", None, "*".to_string()).await.unwrap();
        println!("got cards!");

        _ = find_and_replace(&Client::new(), "", "", "Reading", cards, false, Some(false), clone).await;
        println!("{:?} seconds", now.elapsed().as_secs());
    });

    let clone = arc.clone();
    let b = tokio::task::spawn(async move {
        loop {
            if poll_count(clone.clone()).await >= 0 {
                println!("Value: {:?}", poll_count(clone.clone()).await);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    });

    a.await.unwrap();
    b.await.unwrap();
}
