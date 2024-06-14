use crate::types::YouTubeSearchResult;
use log::info;
use rusty_ytdl::blocking::search;
use rusty_ytdl::blocking::search::SearchResult;
use rusty_ytdl::blocking::search::YouTube;

pub async fn _see_content(search: String) -> Vec<rusty_ytdl::blocking::search::SearchResult> {
    let yt = YouTube::new().unwrap();
    let options = rusty_ytdl::blocking::search::SearchOptions {
        search_type: rusty_ytdl::blocking::search::SearchType::All,
        limit: 20, // configurable at some point !?
        safe_search: true,
    };
    yt.search(search, Some(&options)).unwrap()
}

pub async fn content_to_text(
    search: String,
    videos: bool,
    playlists: bool,
) -> Vec<YouTubeSearchResult> {
    // we use our own youtube search result so we dont need to fetch the videos in the playlist
    // each time the scrollable is re-rendered
    let yt = YouTube::new().unwrap();
    // search based on the checkboxes
    let search_type = if videos && playlists {
        search::SearchType::All
    } else if videos {
        search::SearchType::Video
    } else {
        search::SearchType::Playlist
    };
    let options = search::SearchOptions {
        search_type,
        limit: 20,
        safe_search: true,
    };
    let mut ret = vec![];
    let results = yt.search(search, Some(&options)).unwrap();
    for result in results {
        match result {
            SearchResult::Video(vid) => {
                let n = YouTubeSearchResult {
                    // title: vid.title.chars().take(30).collect(),
                    title: vid.title.clone(),
                    author: vid.channel.name.clone(),
                    views: vid.views,
                    duration: Some(vid.duration_raw),
                    videos: None,
                    thumbnail: vid.thumbnails[0].url.clone(),
                    link: vid.url,
                };
                ret.push(n);
            }
            SearchResult::Playlist(playlist) => {
                // this is required to get the videos, without it, using playlist.videos, returns 0 every time
                let play = search::Playlist::get(playlist.url.clone(), None).unwrap();
                let n = YouTubeSearchResult {
                    title: playlist.name.clone(),
                    author: playlist.channel.name.clone(),
                    views: playlist.views,
                    duration: None,
                    videos: Some(format!("{} Videos", play.videos.len())),
                    thumbnail: playlist.thumbnails[0].url.clone(),
                    link: playlist.url,
                };
                ret.push(n)
            }
            SearchResult::Channel(_chn) => {}
        };
    }
    info!("length of returned videos: {}", ret.len());
    ret
}
