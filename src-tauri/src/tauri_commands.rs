use crate::db::models::{download::*, preferences::*, smart_mode::*};
use crate::db::*;
use crate::events_names::*;
use crate::file_system::file_system;
use crate::youtube_downloader::models::video::*;
use crate::youtube_downloader::video;
use tauri::Window;

#[tauri::command]
pub fn show_in_folder(directory: String, file_name: String) {
    file_system::show_in_folder(directory, file_name);
}

#[tauri::command]
pub fn delete_file(directory: String, file_name: String, id: i32) -> Result<(), String> {
    let result = downloads::remove_download_item(id);

    if result.is_ok() {
        let _ = file_system::delete_file(directory, file_name).unwrap_or(());
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
            .emit(RELOAD_SMART_MODE, result.clone().unwrap())
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
    let result = downloads::queue_new_download(new_download_item.clone());

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let new_download = result.unwrap();

    return Ok(new_download);
}

#[tauri::command]
pub async fn queue_parsed_download(options: NewParsedDownloadItem) -> Result<DownloadItem, String> {
    let insert_options = NewDownloadItem {
        url: options.url,
        directory: options.directory,
        format: options.format,
        quality: options.quality,
        quality_label: options.quality_label,
    };
    let insert_result = downloads::queue_new_download(insert_options.clone());

    if insert_result.is_err() {
        return Err(insert_result.unwrap_err());
    }

    let new_id = insert_result.unwrap().id;

    let update_options = UpdateDownloadItem {
        id: new_id,
        title: options.title,
        thumbnail: options.thumbnail,
        file_name: options.file_name,
        length_seconds: options.length_seconds,
        size_in_bytes: options.size_in_bytes,
        approx_duration_ms: options.approx_duration_ms,
    };
    return downloads::update_download_info(update_options);
}

#[tauri::command]
pub async fn parsing_video(
    download_item: DownloadItem,
    window: Window,
) -> Result<DownloadItem, String> {
    let result = downloads::update_download_status(&download_item.id, &"parsing".to_string());

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    window
        .emit(ON_DOWNLOAD_STATUS_CHANGES, result.unwrap())
        .unwrap();

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
        return Err(result.err().unwrap());
    }

    video::download_video(&download_item, window).await;
    return Ok(());
}

#[tauri::command]
pub async fn download_all_pending(window: Window) {
    println!("Checking the pending downloads..");
    let downloads = downloads::get_downloads().unwrap();
    for download in downloads.iter() {
        if download.status == "downloading" {
            video::download_video(download, window.clone()).await;
        }
    }
    println!("Checked the pending downloads");
}
