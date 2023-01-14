use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct SmartMode {
    pub id: i32,
    pub enabled: bool,
    pub format: Option<String>,
    pub quality: Option<String>,
    pub directory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSmartModeOptions {
    pub id: i32,
    pub enabled: bool,
    pub format: Option<String>,
    pub quality: Option<String>,
    pub directory: Option<String>,
}