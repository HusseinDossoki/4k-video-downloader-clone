use rustube::{
    url::Url,
    video_info::player_response::streaming_data::{Quality, QualityLabel},
    Id, Video, VideoDescrambler, VideoFetcher,
};

pub async fn download_youtube_vuideo(video_url: &String) {
    let url = Url::parse(video_url).unwrap();
    let fetcher: VideoFetcher = VideoFetcher::from_url(&url).unwrap();
    let descrambler: VideoDescrambler = fetcher.fetch().await.unwrap();
    let video: Video = descrambler.descramble().unwrap();

    let video_path = video
        .streams()
        .iter()
        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
        .find(|x| x.includes_video_track);

    match video_path {
        Some(data) => {
            data.download_to_dir(get_download_dir()).await.unwrap();
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

fn get_download_dir() -> String {
    return String::from(".");
}
