use crate::db;
use rustube::{
    url::Url,
    video_info::player_response::streaming_data::{Quality, QualityLabel},
    Id, Video, VideoDescrambler, VideoFetcher,
};
use tauri::Window;

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

pub async fn download_youtube_video(
    download_item: &db::models::DownloadItem,
    window: Window,
) {
    let url = Url::parse(&download_item.url).unwrap();
    let fetcher: VideoFetcher = VideoFetcher::from_url(&url).unwrap();
    let descrambler: VideoDescrambler = fetcher.fetch().await.unwrap();
    let video: Video = descrambler.descramble().unwrap();

    let stream = video
        .streams()
        .iter()
        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
        .find(|x| x.includes_video_track);

    match stream {
        Some(data) => {
            // Update info in db
            let params = db::models::UpdateDownloadItemFullInfo {
                id: download_item.id.clone(),
                format: data.mime.to_string().clone(),
                quality: quality_string(&data.quality),
                quality_label: quality_label_string(&data.quality_label.unwrap()),
                size_in_bytes: data.content_length().await.unwrap() as i32,
            };

            db::downloads::update_video_full_info(params.clone()).unwrap();
            window.emit("downloads-changed", true).unwrap();

            let file_path = format!("{}/{}.mp4", &download_item.directory, &download_item.title.clone().unwrap());
            data.download_to(file_path).await.unwrap();
            db::downloads::download_completed(&download_item.id).unwrap();
            window.emit("downloads-changed", true).unwrap();
        }
        None => todo!(),
    }

    // let video_path = video
    //     .streams()
    //     .iter()
    //     .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
    //     .max_by_key(|stream| stream.quality_label)
    //     .unwrap()
    //     .download()
    //     .await
    //     .unwrap();
}

fn quality_string(val: &Quality) -> String {
    let serilazed = serde_json::to_string(val).unwrap().clone();
    return serde_json::from_str::<String>(&serilazed).unwrap();
}
fn quality_label_string(val: &QualityLabel) -> String {
    let serilazed = serde_json::to_string(val).unwrap().clone();
    return serde_json::from_str::<String>(&serilazed).unwrap();
}
