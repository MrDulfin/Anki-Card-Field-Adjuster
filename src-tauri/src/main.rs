// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Client;
use std::io::Error;
use std::sync::atomic::{AtomicI32, Ordering};
use tauri::{LogicalSize, Manager, Size};

use crate::requests::{check_for_cards, deck_names, edit_cards, find_notes, get_models};

mod edits;
mod requests;
mod responses;

pub struct CountState(pub AtomicI32);

#[tauri::command]
async fn get_decks() -> Vec<String> {
    deck_names().await
}
#[tauri::command]
async fn get_notes(deck: String) -> Result<(), String> {
    _ = find_notes(&Client::new(), &deck, None, "".to_string()).await;
    Ok(())
}

#[tauri::command]
async fn find_models() -> Vec<(String, Vec<String>)> {
    let bun = get_models().await.unwrap();
    let mut ny = Vec::new();
    for model in bun {
        ny.push((model.name, model.fields));
    }
    ny
}
#[tauri::command]
async fn poll_count(count: tauri::State<'_, CountState>) -> Result<i32, ()> {
    Ok(count.0.load(Ordering::Acquire))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::Builder::default()
        .manage(CountState(AtomicI32::from(0)))
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            _ = window.set_resizable(false);
            _ = window.set_title("MrDulfin's Anki Card Field Adjuster");
            _ = window.set_size(Size::Logical(LogicalSize::new(1250.0, 750.0)));
            dbg!(&window.outer_size(), &window.inner_size());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_decks,
            edit_cards,
            get_notes,
            find_models,
            poll_count,
            check_for_cards
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
