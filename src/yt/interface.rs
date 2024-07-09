use crate::db::insert::add_to_main;
use crate::db::update::update_empty_entries;
use crate::types::{AppError, DatabaseErrors, PungeMusicObject, YouTubeData};
use crate::utils::sep_video;
use itertools::Itertools;
use log::{debug, error, info, warn};
use regex::Regex;
use rusqlite;
use rusty_ytdl::blocking::Video;
// it is assumed that the link passed in here should be checked for it being a playlist.
// so every link should be just downloading one video
// we do need to know if this function was called under the pretext of us knowing the album / playlist title

pub async fn playlist_wrapper(
    link: String,
) -> Result<rusty_ytdl::blocking::search::Playlist, rusty_ytdl::VideoError> {
    rusty_ytdl::blocking::search::Playlist::get(link, None)
}

pub async fn download_interface(
    url: String,
    playlist_title: Option<String>,
    order: usize, // where the song will be inserted in main
) -> Result<YouTubeData, AppError> {
    let vid_opt = rusty_ytdl::VideoOptions {
        quality: rusty_ytdl::VideoQuality::HighestAudio,
        filter: rusty_ytdl::VideoSearchOptions::Audio,
        download_options: rusty_ytdl::DownloadOptions::default(),
        request_options: rusty_ytdl::RequestOptions::default(),
    };
    let video = Video::new_with_options(&url, vid_opt)?; // url check
    info!("playlist_title: {:?}", &playlist_title);
    if check_if_exists(video.get_video_id()) && playlist_title.is_none() {
        // if the entry exists already
        warn!("The video already exists");
        return Err(AppError::DatabaseError(
            DatabaseErrors::DatabaseEntryExistsError,
        ));
    }
    let details = video.get_basic_info().unwrap().video_details;
    let (mp3, jpg) = fetch_json();

    // different cases for videos:
    // 1. normal, title has auth and title in it, separated by " - ", no album
    // 2. auto-gen. title = title, author = author, album = 4th line in description
    // 3. one vid = whole album. title, author = title.split("-")
    // 4. playlist album. playlist.title = album, author = firstvid.author, title = each title
    // ^^^^ is for what the playlist_title param is for
    // 5. assorted playlist? idk if i wanna do this one though, seems hard to distinguish from #4
    // and i never use it..
    // 6. / else: title = title, author = author, album = single. hit no others...

    let youtube_data = if playlist_title.is_none() && details.title.contains(" - ") {
        // #1, has a dash in title, no playlist
        let title_string = details.title.split(" - ").collect_vec();
        let title = title_string[0].to_string(); // first half of the title
        let author = title_string[1].to_string(); // second half. expects: <title> - <artist>
        let album = "Single".to_string();
        let yt_data = YouTubeData {
            title,
            author,
            album,
            url: url.clone(),
        };
        let obj = create_punge_obj(
            video,
            yt_data.clone(),
            String::from("None"),
            jpg,
            mp3,
            details.video_id,
            details.length_seconds.parse::<u32>().unwrap(),
            order,
        )
        .await?;
        add_to_main(obj)?;
        yt_data
    // } {
    } else if playlist_title.is_some() {
        // #4 a playlist, assuming that each vid.title = title and vid.author = author
        let title = details.title;
        let author = details.author;
        let album = playlist_title.unwrap();
        let youtube_data = YouTubeData {
            title,
            author: author.unwrap().name.to_string(),
            album,
            url: url.clone(),
        };
        let obj = create_punge_obj(
            video,
            youtube_data.clone(),
            String::from("None"),
            jpg,
            mp3,
            details.video_id,
            details.length_seconds.parse::<u32>().unwrap(),
            order,
        )
        .await?;
        info!("updating: {}", &obj.title);
        add_to_main(obj)?;
        youtube_data
    } else if description_timestamp_check(details.description.as_str()) {
        // how is this meant to be done ??
        let (album, auth) = if details.title.contains(" - ") {
            let i = details.title.split(" - ").collect_vec();
            (i[0].to_string(), i[0].to_string())
        } else {
            let album = details.title.clone();
            let auth = details.author.unwrap().name;
            (album, auth)
        };
        let yt_data = YouTubeData {
            title: String::from("no title here :)"),
            author: auth,
            album,
            url: url.clone(),
        };
        let temp_punge_obj = create_punge_obj(
            video.clone(),
            yt_data.clone(),
            String::from("none"),
            jpg.clone(),
            mp3.clone(),
            details.video_id.clone(),
            details.length_seconds.parse::<u32>().unwrap(),
            order,
        );
        let punge_iter = sep_video::separate(
            details.description,
            temp_punge_obj.await.unwrap(),
            mp3.clone(),
            details.length_seconds.parse::<usize>().unwrap(),
            order,
        );
        for sub_item in punge_iter {
            add_to_main(sub_item)?;
        }
        yt_data
    // } else if playlist_title.is_some() {
    } else if details.description.starts_with("Provided") {
        // #2 autogens
        // no check for playlist_title, since it doesnt matter for this
        let title = details.title;
        let author = details.author;
        let album = details.description.split('\n').collect_vec()[4].to_string();
        let youtube_data = YouTubeData {
            title,
            author: author.unwrap().name,
            album,
            url: url.clone(),
        };
        let obj = create_punge_obj(
            video,
            youtube_data.clone(),
            String::from("None"),
            jpg,
            mp3,
            details.video_id,
            details.length_seconds.parse::<u32>().unwrap(),
            order,
        )
        .await?;
        add_to_main(obj.clone())?;
        youtube_data
    } else {
        // these if elifs cannot find any recognized format. default to this...
        let youtube_data = YouTubeData {
            title: details.title,
            author: details.author.unwrap().name,
            album: String::from("Single"),
            url: url.clone(),
        };
        let obj = create_punge_obj(
            video,
            youtube_data.clone(),
            String::from("None"),
            jpg,
            mp3,
            details.video_id,
            details.length_seconds.parse::<u32>().unwrap(),
            order,
        )
        .await?;
        add_to_main(obj.clone())?;
        youtube_data
    };
    Ok(youtube_data)
}

pub fn check_if_exists(uniqueid: String) -> bool {
    // maybe should be Result!?
    // checks if the given unique id is found inside the main table. aka: has it been downloaded?
    let conn = rusqlite::Connection::open("main.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM main WHERE uniqueid = ?")
        .unwrap();
    let exists = stmt.exists([uniqueid.clone()]).unwrap();
    drop(stmt);
    conn.close().unwrap();
    debug!(
        "{} does {}exist",
        &uniqueid,
        if exists { "" } else { "not " }
    );
    exists
}

fn fetch_json() -> (String, String) {
    // reason we fetch the json each time instead of having it be a static value is because when the app is open
    // the user can change the json value. So we should probably fetch it each time
    let raw_json = std::fs::File::open("./cache/config.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(raw_json).unwrap();
    let mp3 = json.get("mp3_path").unwrap();
    let jpg = json.get("jpg_path").unwrap();
    let mut mp3 = mp3.as_str().unwrap().to_string();
    let mut jpg = jpg.as_str().unwrap().to_string();
    // ensure that the directories given do end with a slash of some type.
    // probably better to ensure that the user-changed input has this slash. later activity
    if !mp3.ends_with('\\') && !mp3.ends_with('/') {
        mp3.push('/')
    }
    if !jpg.ends_with('\\') && !jpg.ends_with('/') {
        jpg.push('/')
    }
    (mp3, jpg)
}

async fn create_punge_obj(
    vid: Video,
    youtube_data: YouTubeData,
    features: String,
    jpg_dir: String,
    mp3_dir: String,
    vid_id: String,
    vid_length: u32, // pass in this and vid_id to avoid calling .await unnecessarily
    order: usize,    // tells us where in 'main' the object sits
) -> Result<PungeMusicObject, AppError> {
    // downloads the video, thumbnail
    // creates the punge obj for further processing if needed (like one song -> whole album)
    let author = clean_inputs_for_win_saving(clean_author(youtube_data.author));
    let title = clean_inputs_for_win_saving(youtube_data.title);
    let album = clean_inputs_for_win_saving(youtube_data.album);
    let naming_conv = format!("{} - {}{}", author, title, vid_id.clone());
    let jpg_name = format!("{}{}.jpg", jpg_dir, naming_conv);
    let mp3_name = format!("{}{}.mp3", mp3_dir, naming_conv);
    if std::path::Path::new(&mp3_name).exists() {
        // should this be checked for eariler? can we???
        return Err(AppError::DatabaseError(DatabaseErrors::FileExistsError));
    }
    // keep in mind that this will add to db whether it fails or not. which is intended
    download_to_punge(
        vid.clone(),
        mp3_dir,
        jpg_dir,
        mp3_name.clone(),
        jpg_name.clone(),
    )
    .await?;
    Ok(PungeMusicObject {
        title,
        author,
        album,
        features,
        length: vid_length,
        savelocationmp3: mp3_name,
        savelocationjpg: jpg_name,
        datedownloaded: chrono::Local::now().date_naive(),
        lastlistenedto: chrono::Local::now().date_naive(),
        ischild: false, // is changed if obj is passed in to be modified by sep_video.rs
        uniqueid: vid_id,
        plays: 0,
        weight: 0,
        threshold: crate::db::utilities::calc_thres(vid_length as usize) as u16,
        order,
    })
}

pub fn clean_inputs_for_win_saving(to_check: String) -> String {
    // will remove any characters that are not allowed in windows files. Not intended for directory names, only potential filenames
    let mut new_string = String::new();
    let forbidden: Vec<char> = vec!['\\', '/', ':', '*', '?', '\"', '<', '>', '|'];
    for character in to_check.chars() {
        if !forbidden.contains(&character) {
            new_string.push(character)
        }
    }
    new_string
}

async fn download_to_punge(
    vid: Video,
    mp3_path: String,
    _jpg_path: String,
    new_mp3_name: String,
    _new_jpg_name: String, // unused rn
) -> Result<(), AppError> {
    // let old_name = format!("{}{}.webm", mp3_path.clone(), vid.video_details().video_id);
    // first we downlaod it as '.mp4' then ffmpeg it over to mp3
    let id = vid.get_basic_info().unwrap();
    let mp4_name = format!(
        "{}{}.mp4",
        mp3_path.clone(),
        id.video_details.video_id.clone()
    ); // can sometimes be .webm??
    let mp3_name = format!(
        "{}{}.mp3",
        mp3_path.clone(),
        id.video_details.video_id.clone()
    );
    let path_download = std::path::Path::new(mp4_name.as_str());
    debug!("mp3 name: {}  mp4_name: {}", &mp3_name, &mp4_name);
    // we assume that the inputs are sanitized by "clean_input_for_win_saving"
    // the unwrap can fail sometimes. so we loop 5 times, sleeping for 3 seconds inbetween so it will try again
    info!("startin download!");
    let before = std::time::Instant::now();
    match vid.download(path_download) {
        Ok(_t) => {
            info!("Download finsihed in: {:.2?}", before.elapsed());
            // convert the old file to (webm) to mp3 and rename
            let x = std::process::Command::new("ffmpeg.exe")
                .args([
                    "-i",
                    mp4_name.as_str(),
                    "-vn",
                    "-c:a",
                    "libmp3lame",
                    "-b:a",
                    "192k",
                    new_mp3_name.as_str(),
                ])
                .output();
            match x {
                Ok(_t) => {
                    info!("File converted successfully, removing now...");
                    match std::fs::remove_file(mp4_name.clone()) {
                        Ok(_t) => {
                            Ok(()) // if the ffmpeg operation goes well and he file is removed
                        }
                        Err(e) => {
                            error!("File failed to be removed {}", &mp4_name);
                            Err(AppError::FileError(format!("{:?}", e))) // if the ffmpeg operation works, and the file is not removed
                        }
                    }
                }
                Err(e) => {
                    error!("Ffmpeg failed: {:?}", &e);
                    Err(AppError::FfmpegError(e.to_string())) // if the ffmpeg operation fails
                }
            }
        }
        Err(e) => {
            error!("Download failed! {:?}", &e);

            Err(AppError::YoutubeError(format!("Error downloading {}", e)))
        }
    }
}

fn clean_author(author: String) -> String {
    // cleans the authors name of VEVO, Official, and - topic
    let length = author.len();
    if author.ends_with(" - Topic") {
        author[..length - 8].to_string()
    } else if author.ends_with("VEVO") {
        // catches both cases where vevo is either attached to the author or not
        // e.g. : KendrickVEVO | Kendrick VEVO
        let new = author[..length - 4].to_string();
        if new.ends_with(' ') {
            new[..new.len() - 1].to_string()
        } else {
            new
        }
    } else if author.ends_with("Official") {
        author[..length - 9].to_string()
    } else {
        author
    }
}

fn description_timestamp_check(desc: &str) -> bool {
    // answering the question: are the timestamps real?
    // this is also a bit more of an edge case function that failed on kanye "5:30", where there are lyrics in the description that include a timestamp
    // a song will have repeating "timestamps" (in lyrics), legitimate timestamps will not
    // maybe a check to see if timestamps increment??
    let pattern = Regex::new(r"\d*:\d\d").unwrap(); // catches timestamps (10:10, 1:34:51..)
    let mut caught_list: Vec<&str> = pattern.find_iter(desc).map(|mat| mat.as_str()).collect(); // list for all captured regex patterns. We will check if they are all the same
    if caught_list.is_empty() {
        // if the caught list it empty, meaning that there are no timestamps
        false
    } else {
        let init_catch = caught_list[0];
        for catch in &mut caught_list[1..] {
            if init_catch == *catch {
                // (after wrote) i think this is to check each one to see if it matches the first one, so songs that have timestamp in name are not caught
                return false;
            }
        }
        // well, no repeats of the first found, is likely fine then
        true
    }
}
