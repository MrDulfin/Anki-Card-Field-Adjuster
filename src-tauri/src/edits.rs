use std::{collections::HashMap, sync::atomic::Ordering};

use reqwest::{Client, Error};

use crate::requests::{get_req, notes_info, NoteInput, Params, ReqType, Request};
use crate::CountState;
pub async fn replace_whole_fields(
    client: &Client,
    cards: Vec<i64>,
    field: &str,
    replace: &str,
    count: tauri::State<'_, CountState>,
) -> Result<(), Error> {
    for (i, card) in cards.iter().enumerate() {
        let mut field2: HashMap<String, String> = HashMap::with_capacity(1);

        field2.insert(field.to_string(), replace.to_string());

        let request: Request = Request {
            action: "updateNoteFields".to_string(),
            version: 6,
            params: Some(Params {
                note: Some(NoteInput {
                    id: card.to_owned(),
                    fields: field2,
                }),
                ..Params::default()
            }),
        };
        _ = get_req(ReqType::None, client, request).await;
        count.0.store(i as i32, Ordering::Release);
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
    count: tauri::State<'_, CountState>,
) -> Result<(), Error> {
    let notes_input: Vec<NoteInput> = cards
        .iter()
        .map(|note: &i64| NoteInput {
            id: *note,
            ..Default::default()
        })
        .collect();

    let replace: Vec<String> = notes_info(client, notes_input)
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

        count.0.store(i as i32, Ordering::Release);
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

// #[tokio::test]
// async fn findreplace_test() {
//     let arc = Arc::new(AtomicI32::from(0));

//     let clone = arc.clone();
//     let a = tokio::task::spawn(async move {
//         let now = Instant::now();
//         let cards = find_notes(&Client::new(), "TheMoeWay Tango N5", None, "*".to_string())
//             .await
//             .unwrap();
//         println!("got cards!");
//         _ = find_and_replace(
//             &Client::new(),
//             "",
//             "",
//             "Reading",
//             cards,
//             false,
//             Some(false),
//             count,
//         )
//         .await;
//         println!("{:?} seconds", now.elapsed().as_secs());
//     });

//     let clone = arc.clone();
//     let b = tokio::task::spawn(async move {
//         loop {
//             if poll_count(clone.clone()).await >= 0 {
//                 println!("Value: {:?}", poll_count(clone.clone()).await);
//                 tokio::time::sleep(Duration::from_millis(200)).await;
//             }
//         }
//     });

//     a.await.unwrap();
//     b.await.unwrap();
// }
