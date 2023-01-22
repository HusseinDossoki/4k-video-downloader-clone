use std::path::Path;

use super::models::lookups::*;
use super::models::video::*;
use super::stream;
use crate::db;
use crate::db::models::download::*;
use crate::events_names::*;
use rustube::{url::Url, Video, VideoDescrambler, VideoFetcher};
use tauri::Window;

use strum::IntoEnumIterator;

pub fn get_lookups() -> Lookups {
    let mut formats = Vec::new();
    let mut quality_labels = Vec::new();
    let mut audio_qualities = Vec::new();

    for x in Format::iter() {
        formats.push(x.clone().to_string());
    }

    for x in QualityLabel::iter() {
        quality_labels.push(x.clone().to_string());
    }

    for x in AudioQuality::iter() {
        audio_qualities.push(x.clone().to_string());
    }

    return Lookups {
        formats,
        quality_labels,
        audio_qualities,
    };
}

pub async fn get_video_details(url: &String) -> Result<VideoDetails, String> {
    let url = Url::parse(url).unwrap();
    let fetcher: VideoFetcher = VideoFetcher::from_url(&url).unwrap();
    let descrambler: VideoDescrambler = fetcher.fetch().await.unwrap();
    let video: Video = descrambler.descramble().unwrap();

    let streams = video.streams();

    let mut streams_result = Vec::new();

    for s in streams.iter() {
        if !s.includes_audio_track {
            continue;
        }
        streams_result.push(StreamDetails {
            quality: Some(stream::quality_string(&s.quality.clone())),
            quality_label: Some(stream::quality_label_string(&s.quality_label.clone())),
            format: Some(s.mime.to_string().clone()),
            size_in_bytes: s.content_length().await.unwrap_or(0) as i32,
            approx_duration_ms: s.approx_duration_ms.unwrap_or(0) as i32,
        });
    }

    streams_result.sort_by_key(|x| std::cmp::Reverse(x.size_in_bytes));

    let result = VideoDetails {
        title: video.title().to_string(),
        file_name: get_file_name(video.title().to_string()),
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

pub async fn parsing_video(
    url: &String,
    format: &Option<String>,
    quality: &Option<String>,
    quality_label: &Option<String>,
) -> Result<ParsingVideoResult, String> {
    // Filters
    let mut f_format: Option<Format> = None;
    let mut f_video_quality: Option<Quality> = None;
    let mut f_audio_quality: Option<AudioQuality> = None;
    let mut f_quality_label: Option<QualityLabel> = None;

    if format.is_some() {
        f_format = format.as_ref().unwrap().parse::<Format>().ok();
    }
    if quality_label.is_some() {
        f_quality_label = quality_label.as_ref().unwrap().parse::<QualityLabel>().ok();
    }
    if quality.is_some() && format.is_some() {
        if format.as_ref().unwrap().contains("video") {
            f_audio_quality = None;
            f_video_quality = quality.as_ref().unwrap().parse::<Quality>().ok();
        } else {
            f_video_quality = None;
            f_audio_quality = quality.as_ref().unwrap().parse::<AudioQuality>().ok();
        }
    }

    let stream = stream::get_stream(
        &url,
        f_format,
        f_video_quality,
        f_quality_label,
        f_audio_quality,
    )
    .await;

    let result = ParsingVideoResult {
        stream: stream.clone(),
        title: stream.video_details.title.clone(),
        file_name: get_file_name(stream.video_details.title.clone()),
        thumbnail: stream
            .video_details
            .thumbnails
            .first()
            .unwrap()
            .url
            .to_string(),
        length_seconds: stream.video_details.length_seconds as i32,
        size_in_bytes: stream.content_length().await.unwrap_or(0) as i32,
        approx_duration_ms: stream.approx_duration_ms.unwrap_or(0) as i32,
    };

    return Ok(result);
}

pub async fn download_video(download_item: &DownloadItem, window: Window) {
    let stream = parsing_video(
        &download_item.url,
        &Some(download_item.format.clone()),
        &Some(download_item.quality.clone()),
        &Some(download_item.quality_label.clone()),
    )
    .await
    .unwrap()
    .stream;

    let file_path =
        Path::new(&download_item.directory.clone()).join(&download_item.file_name.clone().unwrap());

    let download_id = download_item.id;
    let window2 = window.clone();

    let callback = rustube::Callback::new()
        .connect_on_progress_closure(move |cargs| {
            let download_progress = DownloadProgress {
                id: download_id,
                current_chunk: cargs.current_chunk.clone() as i32,
            };
            window
                .emit(ON_DOWNLOAD_PROGRESS, download_progress)
                .unwrap();
        })
        .connect_on_complete_closure(move |_| {
            let result =
                db::downloads::update_download_status(&download_id, &"downloaded".to_string())
                    .unwrap();
            window2.emit(ON_DOWNLOAD_STATUS_CHANGES, result).unwrap();
        });

    stream
        .download_to_with_callback(file_path, callback)
        .await
        .unwrap();
}

fn get_file_name(title: String) -> String {
    let ext = "mp4";
    let cleaned_title = title
        .replace("|", "")
        .replace("?", "")
        .replace("*", "")
        .replace("/", "")
        .replace("\\", "")
        .replace("\"", "")
        .replace(":", "")
        .replace("<", "")
        .replace(">", "");
    return format!("{cleaned_title}.{ext}");
}
