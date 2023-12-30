use std::collections::HashMap;

use reqwest::{Client, Error};

use crate::requests::{find_notes, get_req, query_send, NoteInput, Params, ReqType, Request};
pub async fn replace_fields(
    client: &Client,
    deck: &str,
    cards_with: Option<String>,
    field: &str,
    replace: &str,
) -> Result<(), Error> {
    let cards = find_notes(
        client,
        deck,
        Some(field),
        match cards_with {
            Some(e) => e,
            None => "".to_string(),
        },
    )
    .await;

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
