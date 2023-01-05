use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SmartMode {
    pub id: i32,
    pub enabled: bool,
    pub format: String,
    pub quality: String,
    pub directory: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSmartMode {
    pub id: i32,
    pub enabled: bool,
    pub format: String,
    pub quality: String,
    pub directory: String,
}