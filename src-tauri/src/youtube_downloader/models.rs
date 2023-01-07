use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubeVideoInfo {
  pub title: String,
  pub length_seconds: u64,
  pub thumbnail: String,
}