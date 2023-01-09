use crate::db::{downloads, models, preferences, smart_mode};
use crate::file_system::file_system;
use crate::youtube_downloader::downloader;
use tauri::Window;

#[tauri::command]
pub fn show_in_folder(path: String) {
    file_system::show_in_folder(path);
}

#[tauri::command]
pub fn get_smart_mode_lookups() -> crate::youtube_downloader::models::Lookups {
    return downloader::get_lookups();
}

#[tauri::command]
pub fn get_smart_mode() -> Result<models::SmartMode, String> {
    return smart_mode::get_smart_mode();
}

#[tauri::command]
pub fn update_smart_mode(
    params: models::UpdateSmartMode,
    window: Window,
) -> Result<models::SmartMode, String> {
    let result = smart_mode::update_smart_mode(params);

    if result.is_ok() {
        window
            .emit("smart-mode-changed", result.clone().unwrap())
            .unwrap();
    }

    return result;
}

#[tauri::command]
pub fn get_general_preferences() -> Result<models::GeneralPreferences, String> {
    return preferences::get_general_preferences();
}

#[tauri::command]
pub fn update_general_preferences(
    params: models::UpdateGeneralPreferences,
) -> Result<models::GeneralPreferences, String> {
    return preferences::update_general_preferences(params);
}

#[tauri::command]
pub fn get_downloads() -> Result<Vec<models::DownloadItem>, String> {
    return downloads::get_downloads();
}

#[tauri::command]
pub async fn download_new_video(
    url: String,
    directory: String,
    window: Window,
) -> Result<i32, String> {
    let insert_result = downloads::add_download_item(url.clone(), directory.clone());

    if insert_result.is_ok() {
        window.emit("downloads-changed", true).unwrap();
    }

    let video_info = downloader::get_video_info(url.clone()).await;
    let update_video_info_params = models::UpdateDownloadItemInfo {
        id: insert_result.clone().unwrap(),
        title: video_info.title.clone(),
        thumbnail: video_info.thumbnail.clone(),
        length_seconds: video_info.length_seconds.clone() as i32,
    };

    let update_result = downloads::update_video_info(update_video_info_params);

    if update_result.is_ok() {
        window.emit("downloads-changed", true).unwrap();
    }

    let new_video = update_result.unwrap();
    downloader::download_youtube_video(&new_video, window)
    .await;

    return insert_result;
}

#[tauri::command]
pub async fn delete_download_item(id: i32, window: Window) -> Result<(), String> {
    let result = downloads::delete_download_item(id);

    if result.is_ok() {
        window.emit("downloads-changed", true).unwrap();
    }

    return result;
}

#[tauri::command]
pub async fn remove_all_downloads(window: Window) {
    let result = downloads::remove_all_downloads();
    if result.is_ok() {
        window.emit("downloads-changed", true).unwrap();
    }
}

#[tauri::command]
pub fn update_download_progress(id: i32, progress: i32, time_left_sec: i32) {
    let _ = downloads::update_download_progress(id, progress, time_left_sec);
}
