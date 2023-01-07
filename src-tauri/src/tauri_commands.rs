use crate::db::{downloads, models, preferences, smart_mode};
use tauri::Window;

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
