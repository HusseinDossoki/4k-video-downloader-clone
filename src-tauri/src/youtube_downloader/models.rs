use serde::{Serialize, Deserialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Serialize, Deserialize, Debug)]
pub struct Lookups {
  pub formats: Vec<String>,
  pub audio_qualities: Vec<String>,
  pub quality_labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubeVideoInfo {
  pub title: String,
  pub length_seconds: u64,
  pub thumbnail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideo {
  pub id: String,
  pub title: String,
  pub thumbnail: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize_enum_str, Serialize_enum_str, strum_macros::EnumIter)]
pub enum Format {
    #[serde(rename = "Any Video")]
    AnyVideo,
    #[serde(rename = "video/mp4")]
    VideoMP4,
    #[serde(rename = "video/mkv")]
    VideoMKV,
    #[serde(rename = "video/webm")]
    VideoWEBM,
    #[serde(rename = "audio/mp4")]
    AudioMP4,
    #[serde(rename = "audio/webm")]
    AudioWEBM,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize_enum_str, Serialize_enum_str, strum_macros::EnumIter)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_LOW", alias = "low")]
    Low,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM", alias = "medium")]
    Medium,
    #[serde(rename = "AUDIO_QUALITY_HIGH", alias = "high")]
    High,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize_enum_str, Serialize_enum_str, strum_macros::EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Tiny,
    Small,
    Medium,
    Large,
    Highres,
    Hd720,
    Hd1080,
    Hd1440,
    Hd2160,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize_enum_str, Serialize_enum_str, strum_macros::EnumIter)]
#[non_exhaustive]
pub enum QualityLabel {
    #[serde(rename = "144p")]
    P144,
    #[serde(rename = "144p HDR")]
    P144HDR,
    #[serde(rename = "144p60 HDR")]
    P144Hz60HDR,
    #[serde(rename = "240p")]
    P240,
    #[serde(rename = "240p HDR")]
    P240HDR,
    #[serde(rename = "240p60 HDR")]
    P240Hz60HDR,
    #[serde(rename = "360p")]
    P360,
    #[serde(rename = "360p HDR")]
    P360HDR,
    #[serde(rename = "360p60")]
    P360Hz60,
    #[serde(rename = "360p60 HDR")]
    P360Hz60HDR,
    #[serde(rename = "480p")]
    P480,
    #[serde(rename = "480p HDR")]
    P480HDR,
    #[serde(rename = "480p60")]
    P480Hz60,
    #[serde(rename = "480p60 HDR")]
    P480Hz60HDR,
    #[serde(rename = "720p")]
    P720,
    #[serde(rename = "720p50")]
    P720Hz50,
    #[serde(rename = "720p60")]
    P720Hz60,
    #[serde(rename = "720p60 HDR")]
    P720Hz60HDR,
    #[serde(rename = "1080p")]
    P1080,
    #[serde(rename = "1080p50")]
    P1080Hz50,
    #[serde(rename = "1080p60")]
    P1080Hz60,
    #[serde(rename = "1080p60 HDR")]
    P1080Hz60HDR,
    #[serde(rename = "1440p")]
    P1440,
    #[serde(rename = "1440p60")]
    P1440Hz60,
    #[serde(rename = "1440p60 HDR")]
    P1440Hz60HDR,
    #[serde(rename = "2160p")]
    P2160,
    #[serde(rename = "2160p60")]
    P2160Hz60,
    #[serde(rename = "2160p60 HDR")]
    P2160Hz60HDR,
    #[serde(rename = "4320p")]
    P4320,
    #[serde(rename = "4320p60")]
    P4320Hz60,
    #[serde(rename = "4320p60 HDR")]
    P4320Hz60HDR,
}
