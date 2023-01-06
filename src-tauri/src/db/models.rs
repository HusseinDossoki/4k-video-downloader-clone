use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SmartMode {
    pub id: i32,
    pub enabled: bool,
    pub format: String,
    pub quality: String,
    pub directory: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSmartMode {
    pub id: i32,
    pub enabled: bool,
    pub format: String,
    pub quality: String,
    pub directory: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct GeneralPreferences {
    pub id: i32,
    pub prevent_system_sleep: bool,
    pub create_playlist_subdirectory: bool,
    pub numerate_playlists_files: bool,
    pub skip_playlists_duplicates: bool,
    pub remove_downloaded_items: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGeneralPreferences {
    pub id: i32,
    pub prevent_system_sleep: bool,
    pub create_playlist_subdirectory: bool,
    pub numerate_playlists_files: bool,
    pub skip_playlists_duplicates: bool,
    pub remove_downloaded_items: bool,
}