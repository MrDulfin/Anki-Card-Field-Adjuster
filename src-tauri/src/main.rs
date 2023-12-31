// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use reqwest::{Client, Error};
// use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::io::Error;
use tauri::Manager;

use crate::requests::*;

mod edits;
mod requests;
mod responses;

#[tauri::command]
async fn get_decks() -> Vec<String> {
    deck_names().await
}
#[tauri::command]
async fn get_notes(deck: String) -> Result<(), String> {
    _ = find_notes(&Client::new(), &deck, None, "".to_string());
    Ok(())
}

#[tauri::command]
async fn query(
    deck: String,
    cards_with: Option<String>,
    field: String,
    replace: String,
    findreplace: bool,
) -> String {
    requests::query_send(deck, cards_with, field, replace, findreplace).await
} //TODO: add findreplace to the front end

#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::Builder::default()
        // .manage(MyState(text.into()))
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            _ = window.set_resizable(false);
            _ = window.set_title("MrDulfin's Anki Card Field Adjuster");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_decks, query, get_notes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
