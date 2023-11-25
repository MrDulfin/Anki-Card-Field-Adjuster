// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

mod requests;



#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::Builder::default()
        // .manage(MyState(text.into()))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())

}
