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
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;
use tauri_commands as commands;

use crate::events_names::{CHECK_APP_UPDATE, RESUME_DOWNLOADING_ALL_PENDING};

static EVENT_EMITTED: AtomicBool = AtomicBool::new(false);

pub fn is_event_emitted() -> bool {
    EVENT_EMITTED.load(Ordering::Relaxed)
}

pub fn set_event_emitted() {
    EVENT_EMITTED.store(true, Ordering::Relaxed);
}

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
            commands::queue_parsed_download,
            commands::parsing_video,
            commands::download_video,
            commands::download_all_pending,
        ])
        .on_page_load(|app, _| {
            /*
             * Work around to run the command once the app launch
             * "setup, on_page_load Event::Ready are all too early for js listeners to be registered already. We do have plans do explore what we can do in the near future but no promises"
             */
            app.clone().listen("js_listeners_ready", move |_| {
                if is_event_emitted() == false {
                    println!("js listeners are ready");
                    app.clone()
                        .emit_all(RESUME_DOWNLOADING_ALL_PENDING, true)
                        .unwrap();
                    // app.clone().emit_all(CHECK_APP_UPDATE, true).unwrap();
                    set_event_emitted();
                }
            });

            return ();
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
