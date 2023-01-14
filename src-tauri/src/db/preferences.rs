use crate::db::establish_connection;
use crate::db::models::preferences::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn get_preferences() -> Result<Preferences, String> {
    let conn = establish_connection();
    let result = preferences::dsl::preferences.load::<Preferences>(&conn);

    if result.is_err() {
        return Err("Error when fetching the 'preferences' from the database".to_string());
    }

    let binding = result.unwrap();
    let first = binding.first().clone().unwrap();

    return Ok(Preferences {
        id: first.id,
        // General
        threads_number: first.threads_number,
        prevent_system_sleep: first.prevent_system_sleep,
        create_playlist_subdirectory: first.create_playlist_subdirectory,
        numerate_playlists_files: first.numerate_playlists_files,
        skip_playlists_duplicates: first.skip_playlists_duplicates,
        generate_playlists_m3u: first.generate_playlists_m3u,
        embed_video_subtitles: first.embed_video_subtitles,
        search_audio_tags: first.search_audio_tags,
        remove_downloaded_items: first.remove_downloaded_items,
        submit_download_statistics: first.submit_download_statistics,
        install_beta_version: first.install_beta_version,
        language: first.language.clone(),

        // Connection
        speed: first.speed.clone(),

        // Notifications
        show_notification_when_downloads_complete: first.show_notification_when_downloads_complete,
        show_notification_about_new_videos_from_channel_subscriptions: first
            .show_notification_about_new_videos_from_channel_subscriptions,
        play_notification_sound: first.play_notification_sound,
        show_progress_on_dock_icon: first.show_progress_on_dock_icon,
        confirm_app_exit_if_there_are_incomplete_downloads: first
            .confirm_app_exit_if_there_are_incomplete_downloads,
        confirm_before_item_deleting: first.confirm_before_item_deleting,
        confirm_before_subscription_deleting: first.confirm_before_subscription_deleting,
        ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity: first
            .ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity,
        ask_to_download_channel_if_multiple_videos_were_downloaded_from_it: first
            .ask_to_download_channel_if_multiple_videos_were_downloaded_from_it,
    });
}

pub fn update_general_preferences(
    options: UpdateGeneralPreferences,
) -> Result<Preferences, String> {
    let conn = establish_connection();

    use preferences::dsl::{
        create_playlist_subdirectory, embed_video_subtitles, generate_playlists_m3u, id,
        install_beta_version, language, numerate_playlists_files, prevent_system_sleep,
        remove_downloaded_items, search_audio_tags, skip_playlists_duplicates,
        submit_download_statistics, threads_number,
    };

    let res = diesel::update(preferences::dsl::preferences.filter(id.eq(&options.id)))
        .set((
            threads_number.eq(options.threads_number),
            prevent_system_sleep.eq(options.prevent_system_sleep),
            create_playlist_subdirectory.eq(options.create_playlist_subdirectory),
            numerate_playlists_files.eq(options.numerate_playlists_files),
            skip_playlists_duplicates.eq(options.skip_playlists_duplicates),
            generate_playlists_m3u.eq(options.generate_playlists_m3u),
            embed_video_subtitles.eq(options.embed_video_subtitles),
            search_audio_tags.eq(options.search_audio_tags),
            remove_downloaded_items.eq(options.remove_downloaded_items),
            submit_download_statistics.eq(options.submit_download_statistics),
            install_beta_version.eq(options.install_beta_version),
            language.eq(options.language),
        ))
        .execute(&conn);

    if res.is_err() {
        return Err("Error when updating 'general preferences' config in the database".to_string());
    }

    let updated = preferences::dsl::preferences
        .filter(id.eq(&options.id))
        .first::<Preferences>(&conn)
        .expect("'general preferences' not found");

    return Ok(updated);
}

pub fn update_notifications_preferences(
    options: UpdateNotificationsPreferences,
) -> Result<Preferences, String> {
    let conn = establish_connection();

    use preferences::dsl::{
        ask_to_download_channel_if_multiple_videos_were_downloaded_from_it,
        ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity,
        confirm_app_exit_if_there_are_incomplete_downloads, confirm_before_item_deleting,
        confirm_before_subscription_deleting, id, play_notification_sound,
        show_notification_about_new_videos_from_channel_subscriptions,
        show_notification_when_downloads_complete, show_progress_on_dock_icon,
    };

    let res = diesel::update(preferences::dsl::preferences.filter(id.eq(&options.id)))
        .set((
            show_notification_when_downloads_complete
                .eq(options.show_notification_when_downloads_complete),
            show_notification_about_new_videos_from_channel_subscriptions
                .eq(options.show_notification_about_new_videos_from_channel_subscriptions),
            play_notification_sound.eq(options.play_notification_sound),
            show_progress_on_dock_icon.eq(options.show_progress_on_dock_icon),
            confirm_app_exit_if_there_are_incomplete_downloads
                .eq(options.confirm_app_exit_if_there_are_incomplete_downloads),
            confirm_before_item_deleting.eq(options.confirm_before_item_deleting),
            confirm_before_subscription_deleting.eq(options.confirm_before_subscription_deleting),
            ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity
                .eq(options.ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity),
            ask_to_download_channel_if_multiple_videos_were_downloaded_from_it
                .eq(options.ask_to_download_channel_if_multiple_videos_were_downloaded_from_it),
        ))
        .execute(&conn);

    if res.is_err() {
        return Err("Error when updating 'notifications preferences' in the database".to_string());
    }

    let updated = preferences::dsl::preferences
        .filter(id.eq(&options.id))
        .first::<Preferences>(&conn)
        .expect("'preferences' not found");

    return Ok(updated);
}
