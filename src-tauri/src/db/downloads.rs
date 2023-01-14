use crate::db::establish_connection;
use crate::db::models::download::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use std::cmp::Reverse;

pub fn get_downloads() -> Result<Vec<DownloadItem>, String> {
    let conn = establish_connection();
    let result = downloads::dsl::downloads.load::<DownloadItem>(&conn);

    if result.is_err() {
        return Err("Error when fetching the 'downloads' from the database".to_string());
    }

    // Order desc by id
    let mut ordered = result.unwrap();
    ordered.sort_by_key(|x| Reverse(x.id));

    return Ok(ordered);
}

pub fn queue_new_download(options: NewDownloadItem) -> Result<DownloadItem, String> {
    let conn = establish_connection();

    let result = diesel::insert_into(downloads::table)
        .values(&options)
        .execute(&conn);

    if result.is_err() {
        return Err("Error saving new download item to the database".to_string());
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

    use downloads::dsl::id;

    let new_download = downloads::dsl::downloads
        .filter(id.eq(&new_id_res.unwrap()))
        .first::<DownloadItem>(&conn)
        .expect("'download' is not found");

    return Ok(new_download);
}

pub fn update_download_info(options: UpdateDownloadItem) -> Result<DownloadItem, String> {
    let conn = establish_connection();

    use downloads::dsl::{
        approx_duration_ms, file_name, id, length_seconds, size_in_bytes, status, thumbnail, title,
    };

    let res = diesel::update(downloads::dsl::downloads.filter(id.eq(&options.id)))
        .set((
            title.eq(options.title),
            file_name.eq(options.file_name),
            thumbnail.eq(options.thumbnail),
            length_seconds.eq(options.length_seconds),
            size_in_bytes.eq(options.size_in_bytes),
            approx_duration_ms.eq(options.approx_duration_ms),
            status.eq("parsed"),
        ))
        .execute(&conn);

    if res.is_err() {
        return Err("Error when updating 'download info' in the database".to_string());
    }

    let updated = downloads::dsl::downloads
        .filter(id.eq(&options.id))
        .first::<DownloadItem>(&conn)
        .expect("'download item' is not found");

    return Ok(updated);
}

pub fn remove_download_item(qid: i32) -> Result<(), String> {
    let conn = establish_connection();

    use downloads::dsl::id;

    let res = diesel::delete(downloads::dsl::downloads.filter(id.eq(&qid))).execute(&conn);

    if res.is_err() {
        return Err("Error when deleting 'download item' from the database".to_string());
    }

    return Ok(());
}

pub fn remove_all_downloads() -> Result<(), String> {
    let conn = establish_connection();

    let res = diesel::sql_query("DELETE FROM downloads").execute(&conn);

    if res.is_err() {
        return Err("Error when deleting 'all download itens' from the database".to_string());
    }

    return Ok(());
}

pub fn update_download_status(qid: &i32, qstatus: &String) -> Result<DownloadItem, String> {
    let conn = establish_connection();

    use downloads::dsl::{id, status};

    let res = diesel::update(downloads::dsl::downloads.filter(id.eq(&qid)))
        .set(status.eq(qstatus))
        .execute(&conn);

    if res.is_err() {
        return Err(
            "Error when updating 'download item' status in the database".to_string(),
        );
    }

    let updated = downloads::dsl::downloads
        .filter(id.eq(&qid))
        .first::<DownloadItem>(&conn)
        .expect("'download item' is not found");

    return Ok(updated);
}
