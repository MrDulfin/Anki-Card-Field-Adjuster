// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use reqwest::{Client, Error};
// use serde::{Deserialize, Serialize};
use std::io::Error;
use tauri::{window, Window, Manager, Size};

use crate::requests::deck_names as deck_names;

mod requests;

#[tauri::command]
async fn get_decks() -> Vec<String> {
    deck_names().await
}

#[tauri::command]
async fn query(deck: String, cards_with: String, field: String, replace: String) {
 requests::query_send(deck, cards_with, field, replace).await;
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    
    tauri::Builder::default()
        // .manage(MyState(text.into()))
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_resizable(false);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_decks, query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())

}
