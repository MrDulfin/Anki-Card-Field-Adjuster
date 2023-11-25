// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use reqwest::{Client, Error};
// use serde::{Deserialize, Serialize};
use std::io::Error;
use crate::requests::deck_names as deck_names;

mod requests;

#[tauri::command]
async fn get_decks() -> Vec<String> {
    deck_names().await
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::Builder::default()
        // .manage(MyState(text.into()))
        .invoke_handler(tauri::generate_handler![get_decks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())

}
