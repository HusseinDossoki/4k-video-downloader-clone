#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use diesel_migrations::embed_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations");
pub mod db;
pub mod file_system;
pub mod schema;
mod tauri_commands;
pub mod youtube_downloader;
use tauri_commands as commands;

fn main() {
    // Apply the database migrations
    let conn = db::establish_connection();
    embedded_migrations::run(&conn).expect("Error migrating");

    // Tauri config
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_smart_mode,
            commands::update_smart_mode,
            commands::get_general_preferences,
            commands::update_general_preferences,
            commands::get_downloads,
            commands::download_new_video,
            commands::delete_download_item,
            commands::show_in_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
