use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideo {
    pub video_id: String,
    pub title: String,
    pub thumbnail: String,
}