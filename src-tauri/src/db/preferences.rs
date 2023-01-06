use crate::db::establish_connection;
use crate::db::models;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn get_general_preferences() -> Result<models::GeneralPreferences, String> {
    let conn = establish_connection();
    let result = preferences::dsl::preferences.load::<models::GeneralPreferences>(&conn);

    if result.is_err() {
        return Err(
            "Error when fetching the 'general preferences' config from the database".to_string(),
        );
    }

    let binding = result.unwrap();
    let first = binding.first().clone().unwrap();

    return Ok(models::GeneralPreferences {
        id: first.id,
        prevent_system_sleep: first.prevent_system_sleep.clone(),
        create_playlist_subdirectory: first.create_playlist_subdirectory.clone(),
        numerate_playlists_files: first.numerate_playlists_files.clone(),
        skip_playlists_duplicates: first.skip_playlists_duplicates.clone(),
        remove_downloaded_items: first.remove_downloaded_items.clone(),
    });
}

pub fn update_general_preferences(
    params: models::UpdateGeneralPreferences,
) -> Result<models::GeneralPreferences, String> {
    let conn = establish_connection();

    use preferences::dsl::{
        create_playlist_subdirectory, id, numerate_playlists_files, prevent_system_sleep,
        remove_downloaded_items, skip_playlists_duplicates,
    };

    let res = diesel::update(preferences::dsl::preferences.filter(id.eq(&params.id)))
        .set((
            prevent_system_sleep.eq(params.prevent_system_sleep),
            create_playlist_subdirectory.eq(params.create_playlist_subdirectory),
            numerate_playlists_files.eq(params.numerate_playlists_files),
            skip_playlists_duplicates.eq(params.skip_playlists_duplicates),
            remove_downloaded_items.eq(params.remove_downloaded_items),
        ))
        .execute(&conn);

    if res.is_err() {
        return Err("Error when updating 'general preferences' config in the database".to_string());
    }

    let updated = preferences::dsl::preferences
        .filter(id.eq(&params.id))
        .first::<models::GeneralPreferences>(&conn)
        .expect("'general preferences' not found");

    return Ok(updated);
}
