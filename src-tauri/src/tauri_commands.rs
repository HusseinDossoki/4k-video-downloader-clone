use crate::db::models::{download::*, preferences::*, smart_mode::*};
use crate::db::*;
use crate::events_names::*;
use crate::file_system::file_system;
use crate::youtube_downloader::models::video::*;
use crate::youtube_downloader::video;
use tauri::Window;

#[tauri::command]
pub fn show_in_folder(path: String) {
    file_system::show_in_folder(path);
}

#[tauri::command]
pub fn delete_file(path: String, id: i32) -> Result<(), String> {
    let result = downloads::remove_download_item(id);

    if result.is_ok() {
        let _ = file_system::delete_file(path).unwrap();
    }

    return result;
}

#[tauri::command]
pub async fn remove_download_item(id: i32) -> Result<(), String> {
    return downloads::remove_download_item(id);
}

#[tauri::command]
pub async fn remove_all_downloads() -> Result<(), String> {
    return downloads::remove_all_downloads();
}

#[tauri::command]
pub fn get_smart_mode_lookups() -> Lookups {
    return video::get_lookups();
}

#[tauri::command]
pub fn get_smart_mode() -> Result<SmartMode, String> {
    return smart_mode::get_smart_mode();
}

#[tauri::command]
pub fn update_smart_mode(
    params: UpdateSmartModeOptions,
    window: Window,
) -> Result<SmartMode, String> {
    let result = smart_mode::update_smart_mode(params);

    if result.is_ok() {
        window
            .emit("smart-mode-changed", result.clone().unwrap())
            .unwrap();
    }

    return result;
}

#[tauri::command]
pub fn get_preferences() -> Result<Preferences, String> {
    return preferences::get_preferences();
}

#[tauri::command]
pub fn update_general_preferences(params: UpdateGeneralPreferences) -> Result<Preferences, String> {
    return preferences::update_general_preferences(params);
}

#[tauri::command]
pub fn update_notifications_preferences(
    params: UpdateNotificationsPreferences,
) -> Result<Preferences, String> {
    return preferences::update_notifications_preferences(params);
}

#[tauri::command]
pub fn get_downloads() -> Result<Vec<DownloadItem>, String> {
    return downloads::get_downloads();
}

#[tauri::command]
pub async fn get_video_details(url: String) -> Result<VideoDetails, String> {
    return video::get_video_details(&url).await;
}

#[tauri::command]
pub async fn queue_new_download(
    new_download_item: NewDownloadItem,
) -> Result<DownloadItem, String> {
    return downloads::queue_new_download(new_download_item.clone());
}

#[tauri::command]
pub async fn parsing_video(download_item: DownloadItem) -> Result<DownloadItem, String> {
    let parsing_result = video::parsing_video(
        &download_item.url,
        &Some(download_item.format.clone()),
        &Some(download_item.quality.clone()),
        &Some(download_item.quality_label.clone()),
    )
    .await
    .unwrap();

    let update_options = UpdateDownloadItem {
        id: download_item.id.clone(),
        title: parsing_result.title,
        file_name: parsing_result.file_name,
        thumbnail: parsing_result.thumbnail,
        length_seconds: parsing_result.length_seconds,
        size_in_bytes: parsing_result.size_in_bytes,
        approx_duration_ms: parsing_result.approx_duration_ms,
    };
    let update_result = downloads::update_download_info(update_options).unwrap();

    return Ok(update_result);
}

#[tauri::command]
pub async fn download_video(download_item: DownloadItem, window: Window) -> Result<(), String> {
    let result = downloads::update_download_status(&download_item.id, &"downloading".to_string());
    
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    video::download_video(&download_item, window).await;
    return Ok(());
}
