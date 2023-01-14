
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Lookups {
    pub formats: Vec<String>,
    pub audio_qualities: Vec<String>,
    pub quality_labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoDetails {
    pub title: String,
    pub length_seconds: u64,
    pub thumbnail: String,
    pub video_streams: Vec<StreamDetails>,
    pub audio_streams: Vec<StreamDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamDetails {
    pub size_in_bytes: i32,
    pub format: Option<String>,
    pub quality: Option<String>,
    pub quality_label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParsingVideoResult {
    pub stream: rustube::Stream,
    pub title: String,
    pub file_name: String,
    pub thumbnail: String,
    pub length_seconds: i32,
    pub size_in_bytes: i32,
    pub approx_duration_ms: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadProgress {
    pub id: i32,
    pub current_chunk: i32,
}