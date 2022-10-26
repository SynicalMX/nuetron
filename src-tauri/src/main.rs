#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod types;
use types::Position

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create(node: str) {
    if (node === "Position") {
        return Position::new()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
