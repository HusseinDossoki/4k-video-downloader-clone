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
pub mod events_names;
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
            commands::show_in_folder,
            commands::delete_file,
            commands::remove_download_item,
            commands::remove_all_downloads,
            commands::get_smart_mode_lookups,
            commands::get_smart_mode,
            commands::update_smart_mode,
            commands::get_preferences,
            commands::update_general_preferences,
            commands::update_notifications_preferences,
            commands::get_downloads,
            commands::get_video_details,
            commands::queue_new_download,
            commands::parsing_video,
            commands::download_video,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
