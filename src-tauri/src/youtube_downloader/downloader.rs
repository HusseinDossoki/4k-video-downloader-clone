use super::helpers;
use super::models;
use crate::db;
use rustube::{
    url::Url, video_info::player_response::streaming_data, Id, Video, VideoDescrambler,
    VideoFetcher,
};
use tauri::Window;

use strum::IntoEnumIterator;

pub fn get_lookups() -> models::Lookups {
    let mut formats = Vec::new();
    let mut quality_labels = Vec::new();
    let mut audio_qualities = Vec::new();

    for x in models::Format::iter() {
        formats.push(x.clone().to_string());
    }

    for x in models::QualityLabel::iter() {
        quality_labels.push(x.clone().to_string());
    }

    for x in models::AudioQuality::iter() {
        audio_qualities.push(x.clone().to_string());
    }

    return models::Lookups {
        formats,
        quality_labels,
        audio_qualities,
    };
}

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
        approx_duration_ms: stream.approx_duration_ms.unwrap() as i32,
    };
    db::downloads::update_video_full_info(params.clone()).unwrap();
    window.emit("downloads-changed", true).unwrap();

    let file_path = format!(
        "{}/{}.mp4",
        &download_item.directory,
        &download_item.title.clone().unwrap()
    );

    let download_id = download_item.id;
    let window2 = window.clone();

    let callback = rustube::Callback::new()
        .connect_on_progress_closure(move |cargs| {
            // db::downloads::update_download_progress(&download_id, cargs.current_chunk as i32).unwrap(); // "database is locked"

            let p = models::ProgressInfo {
                id: download_id,
                current_chunk: cargs.current_chunk.clone() as i32,
            };
            window.emit("download-progress", p).unwrap();
        })
        .connect_on_complete_closure(move |_| {
            db::downloads::download_completed(&download_id).unwrap();
            window2.emit("downloads-changed", true).unwrap();
        });

    stream
        .download_to_with_callback(file_path, callback)
        .await
        .unwrap();
}

pub async fn get_video_details(url: &String) -> Result<models::VideoDetails, String> {
    let url = Url::parse(url).unwrap();
    let fetcher: VideoFetcher = VideoFetcher::from_url(&url).unwrap();
    let descrambler: VideoDescrambler = fetcher.fetch().await.unwrap();
    let video: Video = descrambler.descramble().unwrap();

    let streams = video.streams();

    let mut streams_result = Vec::new();

    for s in streams.iter() {
        streams_result.push(models::VideoDetailsStream {
            quality: Some(helpers::quality_string(&s.quality.clone())),
            quality_label: Some(helpers::quality_label_string(&s.quality_label.clone())),
            format: Some(s.mime.to_string().clone()),
            size_in_bytes: s.content_length().await.unwrap_or(0) as i32,
        });
    }

    streams_result.sort_by_key(|x| std::cmp::Reverse(x.size_in_bytes));

    let result = models::VideoDetails {
        title: video.title().to_string(),
        thumbnail: video
            .video_details()
            .thumbnails
            .first()
            .unwrap()
            .url
            .to_string(),
        length_seconds: video.video_details().length_seconds,
        video_streams: streams_result
            .clone()
            .into_iter()
            .filter(|x| x.format.clone().unwrap().contains(&"video".to_string()))
            .collect(),
        audio_streams: streams_result
            .clone()
            .into_iter()
            .filter(|x| x.format.clone().unwrap().contains(&"audio".to_string()))
            .collect(),
    };

    return Ok(result);
}
