#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod commands;
mod database;
mod scanner;
mod ocr;
mod gemini;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error mientras se ejecutaba la aplicación Tauri");
}