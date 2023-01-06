pub mod models;
pub mod smart_mode;
use diesel::prelude::*;

use std::path;

use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let _env = env::var("ENV");

    match _env {
        Ok(_env) => {
            let database_url = &env::var("DATABASE_URL").unwrap();

            SqliteConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", &database_url))
        }
        Err(_) => {
            let home_dir = &tauri::api::path::home_dir().unwrap();
            let app_dir = home_dir.join(".clone-4k-video-downloader");
            std::fs::create_dir_all(&app_dir).unwrap();

            let database_url = path::Path::new(&app_dir).join("db.sqlite");

            let database_url = database_url.to_str().clone().unwrap();

            SqliteConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", &database_url))
        }
    }
}
