use super::models::lookups::*;
use rustube::{
    url::Url, video_info::player_response::streaming_data, Video, VideoDescrambler, VideoFetcher,
};

pub async fn get_stream(
    url: &String,
    format: Option<Format>,
    video_quality: Option<Quality>,
    quality_label: Option<QualityLabel>,
    audio_quality: Option<AudioQuality>,
) -> rustube::Stream {
    let url = Url::parse(url).unwrap();
    let fetcher: VideoFetcher = VideoFetcher::from_url(&url).unwrap();
    let descrambler: VideoDescrambler = fetcher.fetch().await.unwrap();
    let video: Video = descrambler.descramble().unwrap();

    let streams = video.streams();
    let mut filtered_streams = Vec::new();

    for s in streams.iter() {
        if !s.includes_audio_track {
            continue;
        }
        
        if format.is_some() {
            if format.unwrap() == Format::AnyVideo {
                if !s.includes_video_track {
                    continue;
                }
            } else {
                if !s.mime.to_string().eq(&format.unwrap().to_string()) {
                    continue;
                }
            }
        }

        if video_quality.is_some() && quality_string(&s.quality) != video_quality.unwrap().to_string() {
            continue;
        }

        if quality_label.is_some()
            && quality_label_string(&s.quality_label) != quality_label.unwrap().to_string()
        {
            continue;
        }

        if audio_quality.is_some()
            && audio_quality_string(&s.audio_quality) != audio_quality.unwrap().to_string()
        {
            continue;
        }

        filtered_streams.push(s);
    }

    filtered_streams.sort_by_key(|s| std::cmp::Reverse(s.quality_label));
    let stream_res = filtered_streams.first();

    match stream_res {
        Some(data) => return data.clone().clone(),
        None => {
            println!("Get the default best stream");
            let streams = streams.clone();
            let video_path = streams
                .iter()
                .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
                .max_by_key(|stream| stream.quality_label)
                .unwrap();
            return video_path.clone();
        }
    }
}

pub fn quality_string(val: &streaming_data::Quality) -> String {
    let serilazed = serde_json::to_string(val).unwrap().clone();
    return serde_json::from_str::<String>(&serilazed).unwrap();
}
pub fn quality_label_string(val: &Option<streaming_data::QualityLabel>) -> String {
    if val.is_none() {
        return "".to_string();
    }
    let serilazed = serde_json::to_string(val).unwrap().clone();
    return serde_json::from_str::<String>(&serilazed).unwrap();
}
pub fn audio_quality_string(val: &Option<streaming_data::AudioQuality>) -> String {
    if val.is_none() {
        return "".to_string();
    }
    let serilazed = serde_json::to_string(val).unwrap().clone();
    return serde_json::from_str::<String>(&serilazed).unwrap();
}
