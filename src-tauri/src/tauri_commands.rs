use crate::db::{smart_mode, models};

#[tauri::command]
pub fn get_smart_mode() -> Result<models::SmartMode, String> {
  return smart_mode::get_smart_mode();
}

#[tauri::command]
pub fn update_smart_mode(params: models::UpdateSmartMode) -> Result<models::SmartMode, String> {
  return smart_mode::update_smart_mode(params);
}
