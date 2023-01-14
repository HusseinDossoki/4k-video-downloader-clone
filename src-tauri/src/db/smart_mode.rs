use crate::db::establish_connection;
use crate::db::models::smart_mode::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn get_smart_mode() -> Result<SmartMode, String> {
    let conn = establish_connection();
    let result = smartmode::dsl::smartmode.load::<SmartMode>(&conn);

    if result.is_err() {
        return Err("Error when fetching the 'SmartMode' config from the database".to_string());
    }

    let binding = result.unwrap();
    let first = binding.first().clone().unwrap();

    return Ok(SmartMode {
        id: first.id,
        enabled: first.enabled,
        format: first.format.clone(),
        quality: first.quality.clone(),
        directory: first.directory.clone(),
    });
}

pub fn update_smart_mode(options: UpdateSmartModeOptions) -> Result<SmartMode, String> {
    let conn = establish_connection();

    use smartmode::dsl::{directory, enabled, format, id, quality};

    let res = diesel::update(smartmode::dsl::smartmode.filter(id.eq(&options.id)))
        .set((
            format.eq(options.format),
            quality.eq(options.quality),
            directory.eq(options.directory),
            enabled.eq(options.enabled),
        ))
        .execute(&conn);

    if res.is_err() {
        return Err("Error when updating 'SmartMode' config in the database".to_string());
    }

    let updated = smartmode::dsl::smartmode
        .filter(id.eq(&options.id))
        .first::<SmartMode>(&conn)
        .expect("'SmartMode' is not found");

    return Ok(updated);
}
