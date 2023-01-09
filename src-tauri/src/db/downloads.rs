use crate::db::establish_connection;
use crate::db::models;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use std::cmp::Reverse;

pub fn get_downloads() -> Result<Vec<models::DownloadItem>, String> {
    let conn = establish_connection();
    let result = downloads::dsl::downloads.load::<models::DownloadItem>(&conn);

    if result.is_err() {
        return Err("Error when fetching the 'downloads' from the database".to_string());
    }

    // Order desc by id
    let mut ordered = result.unwrap();
    ordered.sort_by_key(|x| Reverse(x.id));

    return Ok(ordered);
}

pub fn add_download_item(url: String, directory: String) -> Result<i32, String> {
    let conn = establish_connection();

    let new_download = models::NewDownloadItem { url, directory };

    let result = diesel::insert_into(downloads::table)
        .values(&new_download)
        .execute(&conn);

    if result.is_err() {
        return Err("Error saving new download item".to_string());
    }

    /*
     * Return the new id
     * Note that if the sqlite file is opened by another process, it may cause a database lock error
     */
    no_arg_sql_function!(
        last_insert_rowid,
        diesel::sql_types::Integer,
        "Represents the SQL last_insert_row() function"
    );

    let new_id_res = diesel::select(last_insert_rowid).get_result::<i32>(&conn);
    if new_id_res.is_err() {
        return Err("Error when fetching the last_insert_rowid".to_string());
    }

    return Ok(new_id_res.unwrap());
}

pub fn update_video_info(
    params: models::UpdateDownloadItemInfo,
) -> Result<models::DownloadItem, String> {
    let conn = establish_connection();

    use downloads::dsl::{id, length_seconds, status, thumbnail, title};

    let res = diesel::update(downloads::dsl::downloads.filter(id.eq(&params.id)))
        .set((
            title.eq(params.title),
            thumbnail.eq(params.thumbnail),
            length_seconds.eq(params.length_seconds),
            status.eq("inprogress"),
        ))
        .execute(&conn);

    if res.is_err() {
        return Err("Error when updating 'video info' in the database".to_string());
    }

    let updated = downloads::dsl::downloads
        .filter(id.eq(&params.id))
        .first::<models::DownloadItem>(&conn)
        .expect("'video' not found");

    return Ok(updated);
}

pub fn update_video_full_info(
    params: models::UpdateDownloadItemFullInfo,
) -> Result<models::DownloadItem, String> {
    let conn = establish_connection();

    use downloads::dsl::{format, id, quality, quality_label, size_in_bytes, approx_duration_ms};

    let res = diesel::update(downloads::dsl::downloads.filter(id.eq(&params.id)))
        .set((
            quality.eq(params.quality),
            quality_label.eq(params.quality_label),
            size_in_bytes.eq(params.size_in_bytes),
            format.eq(params.format),
            approx_duration_ms.eq(params.approx_duration_ms),
        ))
        .execute(&conn);

    if res.is_err() {
        return Err("Error when updating 'video full info' in the database".to_string());
    }

    let updated = downloads::dsl::downloads
        .filter(id.eq(&params.id))
        .first::<models::DownloadItem>(&conn)
        .expect("'video' not found");

    return Ok(updated);
}

pub fn delete_download_item(qid: i32) -> Result<(), String> {
    let conn = establish_connection();

    use downloads::dsl::id;

    let res = diesel::delete(downloads::dsl::downloads.filter(id.eq(&qid))).execute(&conn);

    if res.is_err() {
        return Err("Error when deleting 'video' from the database".to_string());
    }

    return Ok(());
}

pub fn remove_all_downloads() -> Result<(), String> {
    let conn = establish_connection();

    let res = diesel::sql_query("DELETE FROM downloads").execute(&conn);

    if res.is_err() {
        return Err("Error when deleting 'all videos' from the database".to_string());
    }

    return Ok(());
}

pub fn download_completed(qid: &i32) -> Result<(), String> {
    let conn = establish_connection();

    use downloads::dsl::{id, status};

    let res = diesel::update(downloads::dsl::downloads.filter(id.eq(&qid)))
        .set(status.eq("downloaded"))
        .execute(&conn);

    if res.is_err() {
        return Err(
            "Error when updating 'video' status to 'downloaded' in the database".to_string(),
        );
    }

    return Ok(());
}
