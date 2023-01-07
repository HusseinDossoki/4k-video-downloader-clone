use std::cmp::Reverse;

use crate::db::establish_connection;
use crate::db::models;
use crate::schema::*;
use diesel::RunQueryDsl;

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
