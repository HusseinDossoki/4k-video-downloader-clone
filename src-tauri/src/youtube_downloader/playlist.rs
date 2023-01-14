use super::models::playlist::*;
use futures::StreamExt;

pub async fn get_playlist_videos(playlist_id: String) -> Result<Vec<PlaylistVideo>, String> {
    let client = ytextract::Client::new();

    // Request the playlist
    let playlist = client.playlist(playlist_id.parse().unwrap()).await.unwrap();

    // Request the videos
    let videos = playlist.videos();

    futures::pin_mut!(videos);

    let mut result: Vec<PlaylistVideo> = Vec::new();

    while let Some(item) = videos.next().await {
        match item {
            Ok(video) => {
                result.push(PlaylistVideo {
                    video_id: video.id().to_string(),
                    title: video.title().to_string(),
                    thumbnail: video.thumbnails().first().unwrap().url.to_string(),
                });
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    return Ok(result);
}
