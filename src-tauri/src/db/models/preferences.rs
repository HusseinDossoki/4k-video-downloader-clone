use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub id: i32,
    // General
    pub threads_number: i32,
    pub prevent_system_sleep: bool,
    pub create_playlist_subdirectory: bool,
    pub numerate_playlists_files: bool,
    pub skip_playlists_duplicates: bool,
    pub generate_playlists_m3u: bool,
    pub embed_video_subtitles: bool,
    pub search_audio_tags: bool,
    pub remove_downloaded_items: bool,
    pub submit_download_statistics: bool,
    pub install_beta_version: bool,
    pub language: String,

    // Connection
    pub speed: String,
    
    // Notifications
    pub show_notification_when_downloads_complete: bool,
    pub show_notification_about_new_videos_from_channel_subscriptions: bool,
    pub play_notification_sound: bool,
    pub show_progress_on_dock_icon: bool,
    pub confirm_app_exit_if_there_are_incomplete_downloads: bool,
    pub confirm_before_item_deleting: bool,
    pub confirm_before_subscription_deleting: bool,
    pub ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity: bool,
    pub ask_to_download_channel_if_multiple_videos_were_downloaded_from_it: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGeneralPreferences {
    pub id: i32,
    pub threads_number: i32,
    pub prevent_system_sleep: bool,
    pub create_playlist_subdirectory: bool,
    pub numerate_playlists_files: bool,
    pub skip_playlists_duplicates: bool,
    pub generate_playlists_m3u: bool,
    pub embed_video_subtitles: bool,
    pub search_audio_tags: bool,
    pub remove_downloaded_items: bool,
    pub submit_download_statistics: bool,
    pub install_beta_version: bool,
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNotificationsPreferences {
    pub id: i32,
    pub show_notification_when_downloads_complete: bool,
    pub show_notification_about_new_videos_from_channel_subscriptions: bool,
    pub play_notification_sound: bool,
    pub show_progress_on_dock_icon: bool,
    pub confirm_app_exit_if_there_are_incomplete_downloads: bool,
    pub confirm_before_item_deleting: bool,
    pub confirm_before_subscription_deleting: bool,
    pub ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity: bool,
    pub ask_to_download_channel_if_multiple_videos_were_downloaded_from_it: bool,
}
