use crate::db::establish_connection;
use crate::db::models;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn get_downloads() -> Result<Vec<models::DownloadItem>, String> {
    let conn = establish_connection();
    let result = downloads::dsl::downloads.load::<models::DownloadItem>(&conn);

    if result.is_err() {
        return Err("Error when fetching the 'downloads' from the database".to_string());
    }

    return Ok(result.unwrap());
}
