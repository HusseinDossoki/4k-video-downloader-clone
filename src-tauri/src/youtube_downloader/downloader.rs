use crate::db::downloads;
use rustube::{
    url::Url,
    video_info::player_response::streaming_data::{Quality, QualityLabel},
    Id, Video, VideoDescrambler, VideoFetcher, VideoInfo,
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
    db_id: &i32,
    video_url: &String,
    directory: &String,
    title: &String,
    window: Window,
) {
    let url = Url::parse(video_url).unwrap();
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
            let size_in_bytes = data.content_length().await.unwrap();
            downloads::set_file_size(&db_id, size_in_bytes).unwrap();
            window.emit("downloads-changed", true).unwrap();

            let file_path = format!("{}/{}.mp4", directory, title);
            data.download_to(file_path).await.unwrap();
            downloads::download_completed(&db_id).unwrap();
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
