use crate::db;
use rustube::{Id, VideoFetcher};
use tauri::Window;

use super::helpers;
use super::models;

pub async fn get_video_info(url: String) -> models::YoutubeVideoInfo {
    let id = Id::from_raw(&url).unwrap();
    let descrambler = VideoFetcher::from_id(id.into_owned())
        .unwrap()
        .fetch()
        .await
        .unwrap();

    let video_info = descrambler.video_info();

    return models::YoutubeVideoInfo {
        title: video_info.player_response.video_details.title.clone(),
        length_seconds: video_info
            .player_response
            .video_details
            .length_seconds
            .clone(),
        thumbnail: video_info
            .player_response
            .video_details
            .thumbnails
            .first()
            .unwrap()
            .url
            .clone(),
    };
}

pub async fn download_youtube_video(download_item: &db::models::DownloadItem, window: Window) {
    let smart_mode = db::smart_mode::get_smart_mode().unwrap();

    // Filters
    let f_format = smart_mode.format.parse::<models::Format>().ok();
    let mut f_quality_label = smart_mode.quality.parse::<models::QualityLabel>().ok();
    let mut f_audio_quality = smart_mode.quality.parse::<models::AudioQuality>().ok();
    if smart_mode.format.contains("video") {
        f_audio_quality = None;
    } else {
        f_quality_label = None;
    }

    let stream = helpers::get_stream(
        &download_item.url,
        f_format,
        None,
        f_quality_label,
        f_audio_quality,
    )
    .await;

    // Update info in db
    let params = db::models::UpdateDownloadItemFullInfo {
        id: download_item.id.clone(),
        format: stream.mime.to_string().clone(),
        quality: helpers::quality_string(&stream.quality),
        quality_label: helpers::quality_label_string(&stream.quality_label),
        size_in_bytes: stream.content_length().await.unwrap() as i32,
    };
    db::downloads::update_video_full_info(params.clone()).unwrap();
    window.emit("downloads-changed", true).unwrap();

    let file_path = format!(
        "{}/{}.mp4",
        &download_item.directory,
        &download_item.title.clone().unwrap()
    );
    stream.download_to(file_path).await.unwrap();
    db::downloads::download_completed(&download_item.id).unwrap();
    window.emit("downloads-changed", true).unwrap();
}
