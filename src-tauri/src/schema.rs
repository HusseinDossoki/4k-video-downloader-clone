// @generated automatically by Diesel CLI.

diesel::table! {
    preferences (id) {
        id -> Integer,
        prevent_system_sleep -> Bool,
        create_playlist_subdirectory -> Bool,
        numerate_playlists_files -> Bool,
        skip_playlists_duplicates -> Bool,
        remove_downloaded_items -> Bool,
    }
}

diesel::table! {
    smartmode (id) {
        id -> Integer,
        enabled -> Bool,
        format -> Text,
        quality -> Text,
        directory -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    preferences,
    smartmode,
);
