use crate::types::YouTubeSearchResult;
use async_std::channel::Send;
use log::info;
use rusty_ytdl::blocking::search;
use rusty_ytdl::blocking::search::SearchResult;
use rusty_ytdl::blocking::search::YouTube;

pub async fn content_to_text(
    search: String,
    videos: bool,
    playlists: bool,
) -> Vec<YouTubeSearchResult> {
    // we use our own youtube search result so we dont need to fetch the videos in the playlist
    // each time the scrollable is re-rendered
    // we begin the search by removing all of the images in the temporary directory (where all of the other images were)
    remove_all_in_temp_dir(); // <-- we force unload the images from the application before this function is called
    let yt = YouTube::new().unwrap();
    // search based on the checkboxes
    let search_type = if videos && playlists {
        search::SearchType::All
    } else if videos {
        search::SearchType::Video
    } else if playlists {
        search::SearchType::Playlist
    } else {
        // funny case where both are unselected.. lol
        search::SearchType::All
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
                crate::utils::image::get_raw_thumbnail_from_link(&vid.id, "./img/temp/").unwrap();
                let n = YouTubeSearchResult {
                    title: vid.title.clone(),
                    author: vid.channel.name.clone(),
                    views: vid.views,
                    duration: Some(vid.duration_raw),
                    videos: None,
                    thumbnail: format!("./img/temp/{}.jpg", &vid.id),
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

fn remove_all_in_temp_dir() {
    // no result, should never fail
    let tmp_path = "./img/temp/";
    for file in std::fs::read_dir(&tmp_path).unwrap() {
        std::fs::remove_file(file.unwrap().path()).unwrap();
    }
}
