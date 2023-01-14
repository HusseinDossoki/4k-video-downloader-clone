// @generated automatically by Diesel CLI.

diesel::table! {
    downloads (id) {
        id -> Integer,
        url -> Text,
        directory -> Text,
        format -> Text,
        quality -> Text,
        quality_label -> Text,
        status -> Text,
        title -> Nullable<Text>,
        file_name -> Nullable<Text>,
        thumbnail -> Nullable<Text>,
        length_seconds -> Nullable<Integer>,
        size_in_bytes -> Nullable<Integer>,
        current_chunk -> Nullable<Integer>,
        approx_duration_ms -> Nullable<Integer>,
    }
}

diesel::table! {
    preferences (id) {
        id -> Integer,
        threads_number -> Integer,
        prevent_system_sleep -> Bool,
        create_playlist_subdirectory -> Bool,
        numerate_playlists_files -> Bool,
        skip_playlists_duplicates -> Bool,
        generate_playlists_m3u -> Bool,
        embed_video_subtitles -> Bool,
        search_audio_tags -> Bool,
        remove_downloaded_items -> Bool,
        submit_download_statistics -> Bool,
        install_beta_version -> Bool,
        language -> Text,
        speed -> Text,
        show_notification_when_downloads_complete -> Bool,
        show_notification_about_new_videos_from_channel_subscriptions -> Bool,
        play_notification_sound -> Bool,
        show_progress_on_dock_icon -> Bool,
        confirm_app_exit_if_there_are_incomplete_downloads -> Bool,
        confirm_before_item_deleting -> Bool,
        confirm_before_subscription_deleting -> Bool,
        ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity -> Bool,
        ask_to_download_channel_if_multiple_videos_were_downloaded_from_it -> Bool,
    }
}

diesel::table! {
    smartmode (id) {
        id -> Integer,
        enabled -> Bool,
        format -> Nullable<Text>,
        quality -> Nullable<Text>,
        directory -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    downloads,
    preferences,
    smartmode,
);
