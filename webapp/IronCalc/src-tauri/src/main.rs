// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use tauri::Manager;

mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::open_xlsx_file,
            commands::save_xlsx_file,
            commands::get_system_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
