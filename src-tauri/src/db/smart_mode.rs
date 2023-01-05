use crate::db::establish_connection;
use crate::db::models;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn get_smart_mode() -> Result<models::SmartMode, String> {
    let conn = establish_connection();
    let result = smartmode::dsl::smartmode.load::<models::SmartMode>(&conn);

    if result.is_err() {
        return Err("Error when fetching the 'SmartMode' config from the database".to_string());
    }

    let binding = result.unwrap();
    let first = binding.first().clone().unwrap();

    return Ok(models::SmartMode {
        id: first.id,
        enabled: first.enabled,
        format: first.format.clone(),
        quality: first.quality.clone(),
        directory: first.directory.clone(),
    });
}

pub fn update_smart_mode(params: models::UpdateSmartMode) -> Result<models::SmartMode, String> {
    let conn = establish_connection();

    use smartmode::dsl::{directory, enabled, format, id, quality};

    let res = diesel::update(smartmode::dsl::smartmode.filter(id.eq(&params.id)))
        .set((
            format.eq(params.format),
            quality.eq(params.quality),
            directory.eq(params.directory),
            enabled.eq(params.enabled),
        ))
        .execute(&conn);

    if res.is_err() {
        return Err("Error when updating 'SmartMode' config in the database".to_string());
    }

    let updated = smartmode::dsl::smartmode
        .filter(id.eq(&params.id))
        .first::<models::SmartMode>(&conn)
        .expect("'SmartMode' not found");

    return Ok(updated);
}
