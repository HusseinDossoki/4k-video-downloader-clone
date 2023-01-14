-- Your SQL goes here
CREATE TABLE
  preferences (
    id INTEGER NOT NULL PRIMARY KEY,
    
    -- General 
    threads_number INTEGER NOT NULL DEFAULT 1,
    prevent_system_sleep BOOLEAN NOT NULL DEFAULT 0,
    create_playlist_subdirectory BOOLEAN NOT NULL DEFAULT 1,
    numerate_playlists_files BOOLEAN NOT NULL DEFAULT 0,
    skip_playlists_duplicates BOOLEAN NOT NULL DEFAULT 0,
    generate_playlists_m3u BOOLEAN NOT NULL DEFAULT 0,
    embed_video_subtitles BOOLEAN NOT NULL DEFAULT 0,
    search_audio_tags BOOLEAN NOT NULL DEFAULT 0,
    remove_downloaded_items BOOLEAN NOT NULL DEFAULT 0,
    submit_download_statistics BOOLEAN NOT NULL DEFAULT 0,
    install_beta_version BOOLEAN NOT NULL DEFAULT 0,
    language VARCHAR NOT NULL DEFAULT 'en',

    -- Connection
    speed VARCHAR NOT NULL DEFAULT 'unlimited',

    -- Notifications
    show_notification_when_downloads_complete BOOLEAN NOT NULL DEFAULT 0,
    show_notification_about_new_videos_from_channel_subscriptions BOOLEAN NOT NULL DEFAULT 0,
    play_notification_sound BOOLEAN NOT NULL DEFAULT 0,
    show_progress_on_dock_icon BOOLEAN NOT NULL DEFAULT 0,
    confirm_app_exit_if_there_are_incomplete_downloads BOOLEAN NOT NULL DEFAULT 0,
    confirm_before_item_deleting BOOLEAN NOT NULL DEFAULT 0,
    confirm_before_subscription_deleting BOOLEAN NOT NULL DEFAULT 0,
    ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity BOOLEAN NOT NULL DEFAULT 0,
    ask_to_download_channel_if_multiple_videos_were_downloaded_from_it BOOLEAN NOT NULL DEFAULT 0
  );

INSERT INTO preferences DEFAULT VALUES;