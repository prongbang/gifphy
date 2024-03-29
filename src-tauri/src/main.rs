// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handler;
mod converter;
mod utils;
mod ffmpeg;
mod command;
mod resources;
mod status;
mod platform;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handler::command::converter,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
