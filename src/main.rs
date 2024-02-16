#![allow(non_snake_case)]

use dioxus::html::geometry::euclid::default;
use dioxus::html::{button, h2, head, input, label, line, link, style};
use dioxus::prelude::*;
use dioxus_desktop::tao::event_loop::{EventLoop, EventLoopWindowTarget};
use dioxus_desktop::tao::window;
use dioxus_desktop::{use_window, Config, LogicalSize, WindowBuilder};
use fermi::prelude::*;

use reqwest::Client;
use serde::de::value;
use std::io::Error;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::requests::{check_for_cards, deck_names, edit_cards, find_notes, get_models};

mod edits;
mod requests;
mod responses;

#[derive(Debug)]
pub struct CountState(pub AtomicI32);

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::new(1000, 750))
                .with_title("MrDulfin's Anki Cards Adjuster")
                .with_resizable(false),
        ),
    );
}

#[allow(clippy::redundant_closure)]
fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let deck_picked = use_state(cx, || String::new());
    let model_picked = use_state(cx, || String::new());
    let model_fields = use_state(cx, || Vec::<String>::new());
    let error_message = use_state(cx, || String::new());
    let mut server = use_state(cx, || "127.0.0.1".to_owned());
    let mut port = use_state(cx, || "8765");
    let decks = get_decks((server.get(), port.get()));
    let models = find_models((server.get(), port.get()));

    let wait_message: &UseState<String> = use_state(cx, || String::new());

    cx.render(rsx! {
        style { include_str!(".\\styles.css") }
        h2 { class: "center", "⚠️BACKUP YOUR DECKS BEFORE USING THIS TOOL⚠️"}
        p {
            class: "center",
            id: "exportMessage",
            "If you have media attached to your anki decks:\nAnki > Export > Fomat: "
        span { class: "code", ".colp" }
            "✅Include Media" }
        // div {
        //     id: "server_port",
        //     div {
        //         id: "server",
        //         p { "server" }
        //         input {
        //             value: "127.0.0.1",
        //             oninput: move |ev| {
        //                 use_set(cx, &SERVER)("{ev}".to_string());
        //                 dbg!(use_read(cx, &SERVER));
        //             }
        //         }
        //     }
        //     div {
        //         id: "port",
        //         p { "port" }
        //         input {
        //             value: "8765",
        //             oninput: move |ev| {
        //                 use_set(cx, &PORT)("{ev}".to_string());
        //                 dbg!(use_read(cx, &PORT));
        //             }
        //         }
        //     }
        // }

        div {
            id: "Confetto",
            div {
                id: "decks",
                class: "dropDownContainer",
                h2 { "Pick a Deck: "}
                div {
                    id: "decksList",
                    class: "dropListClass",
                decks.iter().map(|deck| {
                    // let deck_picked = use_set(cx, &DECK);
                    let name = deck.clone();
                    rsx!(
                        button {
                            class: {
                                if deck_picked.get() == &name {
                                    "decks, selected"
                                }else {
                                    "decks"
                                }
                            },
                            id: "{name}",
                            onclick: move |_| {
                                deck_picked.set(name.to_string());

                            },
                            "{name}"
                        }
                    )})
                }
                }

            div {
                id: "models",
                class: "dropDownContainer",
                h2 { "Pick a Model:" }
                div {
                    id: "modelsList",
                    class: "dropListClass",
                    models.iter().map(|model| {
                        let name = model.0.clone();
                        let fields = model.1.to_owned();
                        rsx!(
                        button {
                            id: "{name}",
                            class: {
                                if model_picked.get() == &name {
                                    "models, selected"
                                }else {
                                    "models"
                                }
                            },
                            onclick: move |_| {
                                model_picked.set(name.clone());
                                model_fields.set(fields.clone()); println!("{name}") },
                            "{name}"
                        }
                    )})
                }
            }

            span {
                id: "wait",
                class: "center",
                "{wait_message}"
            }
            div  {
                class: "findAndReplace",
                form {
                    onsubmit: move |event| 'a: {

                        let deck_picked = deck_picked.get();


                        let data = event.data.values.clone();
                        dbg!(&data);

                        let replace_with = data.get_key_value("replace_with").unwrap().1[0].clone();
                        let field = data.get_key_value("field").unwrap().1[0].clone();
                        let mut cards_with = data.get_key_value("cards_with").unwrap().1[0].clone();


                        let line_breaks: bool = data.get_key_value("line_breaks").is_some();
                        let as_space: bool = data.get_key_value("as_space").is_some();
                        let findreplace = data.get_key_value("remove_whole").is_some();

                        if data.get_key_value("all_cards").is_some() {
                            cards_with = "*".to_string()
                        }
                        if field.is_empty() {
                            error_message.set(String::from("Please enter a field name"));
                            wait_message.set("".to_string());
                            break 'a
                        }else if !model_fields.get().contains(&field) {
                            error_message.set(String::from("Please enter a valid field name"));
                            wait_message.set("".to_string());
                        }


                        let mut cards = Vec::new();

                        dbg!(&replace_with, &field, &cards_with, &line_breaks, &as_space, &findreplace, deck_picked.clone());

                        tokio::runtime::Runtime::new().unwrap().block_on( async {
                            cards = check_for_cards(deck_picked.clone(), cards_with.clone(), field.clone(), (server.get(), port.get())).await.unwrap();
                        });
                        if cards.is_empty() {
                            error_message.set("No cards found!".to_string());
                            wait_message.set("".to_string());
                            break 'a
                        }


                        error_message.set("".to_string());

                        let server = server.get().clone();
                        let port = port.get();


                        wait_message.set("Please Wait...".to_string());
                        tokio::runtime::Runtime::new().unwrap().block_on(async move {
                                edit_cards(cards, field, replace_with, findreplace, cards_with, line_breaks, Some(as_space), (&server, &port)).await.unwrap();
                        });
                        wait_message.set("Done!".to_string());
                    },

                    id: "query",
                    h2 { "For all cards with:" }
                    input {
                        id: "cards_with",
                        name: "cards_with",
                        placeholder: "Leave blank for empty",
                    }
                    input { r#type: "checkbox", id: "all_cards", name: "all_cards", value: "allCards"}
                    label { r#for: "all_cards", "select all cards?"}
                    h2 { "In this field:" }
                    input {
                        id: "field",
                        name: "field",
                    }
                    h2 { "Replace that with:" }
                    input {
                        id: "replace_with",
                        name: "replace_with",
                        placeholder: "Leave blank to erase"
                    }
                    br {}

                    input { r#type: "checkbox", id: "remove_whole", name: "remove_whole", value: "removeWhole" }
                    label { r#for: "remove_whole", "replace entire field?"}
                    br {}
                    br {}

                    input { r#type: "checkbox", id: "line_breaks", name: "line_breaks", value: "lineBreaks" }
                    label { r#for: "line_breaks", "remove line breaks?" }
                    br {}
                    input { r#type: "checkbox", class: "as_space", id: "as_space", name: "as_space", value: "asSpace" }
                    label { class: "as_space", r#for: "line_breaks", "replace line break with a space?",  }
                    br {}

                    input { r#type: "submit", value: "submit"}

                }
            }
            span {
                id: "error",
                class: "center",
                "{error_message}"
            }


        }
    })
}
fn get_decks(server_port: (&str, &str)) -> Vec<String> {
    let a = tokio::runtime::Runtime::new().unwrap();
    a.block_on(async { deck_names(server_port).await })
}

async fn get_notes(deck: String, server_port: (&str, &str)) -> Result<(), String> {
    _ = find_notes(&Client::new(), &deck, None, "".to_string(), server_port).await;
    Ok(())
}

fn find_models(server_port: (&str, &str)) -> Vec<(String, Vec<String>)> {
    let a = tokio::runtime::Runtime::new().unwrap();
    let bun = a.block_on(async { get_models(server_port).await }).unwrap();
    let mut ny = Vec::new();
    for model in bun {
        ny.push((model.name, model.fields));
    }
    ny
}
