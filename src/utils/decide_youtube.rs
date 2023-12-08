use std::path::Path;
// purpose of this file is to have ./youtube_interface.rs pass this file data about the song, and this
// file will decide what is the artist / album / song title
use crate::playliststructs::{AppError, DatabaseErrors, Playlist, PungeMusicObject};
use itertools::Itertools;
use regex::Regex;
use rustube::blocking::Video;
use rustube::url::Url;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Value};
use std::fs;
use std::io::BufReader;
use std::process::Command;

// calls the file that <turns one video & timestamps -> multiple videos> into scope
use crate::utils::sep_video;

use crate::db::fetch;
use crate::db::insert;

// all filenames should follow <artist> - <title><uniqueid>

// struct to serialize for json. represents songs that are added to db but not downloaded
#[derive(Debug, Serialize, Deserialize)]
pub struct Straggler {
    pub id: String,
    pub mp3_path: String,
    pub jpg_path: String,
}

pub async fn begin_playlist(playlist: Playlist) -> Vec<Result<String, AppError>> {
    let mut results: Vec<Result<String, AppError>> = vec![];
    // the dir should be derived from a ./cache/downloadlocation.json["punge_downloads"] this for test
    // if the title of the playlist has the dash then it is: <artist> - <album>
    let (mp3, jpg) = fetch_json();
    // case where the title of playlist is formatted like: <artist> - <album>
    if playlist.title.contains(" - ") {
        let (author, album): (&str, &str) = playlist.title.split(" - ").collect_tuple().unwrap();
        for song in playlist.links {
            loop {
                match loop_handle_playlist(
                    song.clone(),
                    album.to_string(),
                    Some(author.to_string()),
                    jpg.clone(),
                    mp3.clone(),
                ) {
                    Ok(t) => results.push(Ok(t)),
                    Err(e) => {
                        // if 403 error, try again until not 403, await(15)
                        match e {
                            AppError::YoutubeError(_e) => {
                                // this should be the case where a 403 error occurs (from too many requests too frequently)
                                println!("403! sleeping"); // would be nice if we could push this in real-time to the user... maybe a subscription restructure at some point..?
                                async_std::task::sleep(std::time::Duration::from_secs(20)).await;
                            }
                            _ => {
                                results.push(Err(e));
                                break;
                            }
                        }
                    }
                }
            }
            match loop_handle_playlist(
                song,
                album.to_string(),
                Some(author.to_string()),
                jpg.clone(),
                mp3.clone(),
            ) {
                Ok(title_auth) => results.push(Ok(title_auth)),
                Err(e) => {
                    results.push(Err(e))
                    // send to a log?
                }
            }
        }
    }
    // in this case the author is the uploader of the videos, title = title, playlist title = album
    else {
        let album = playlist.title;
        for song in playlist.links {
            match loop_handle_playlist(song, album.to_string(), None, jpg.clone(), mp3.clone()) {
                Ok(title_auth) => results.push(Ok(title_auth)),
                Err(e) => {
                    results.push(Err(e))
                    // send to a log?
                }
            }
        }
    }
    results
}

fn loop_handle_playlist(
    song: String,
    album: String,
    temp_author: Option<String>,
    jpg: String,
    mp3: String,
) -> Result<String, AppError> {
    let url = Url::parse(song.as_str())?;
    let vid = Video::from_url(&url)?;
    let author = match temp_author {
        // if we know the author before hand (begin_playlist)
        Some(auth) => auth,
        None => clean_author(vid.video_details().author.to_string()),
    };
    let punge_obj = create_punge_obj(
        vid.clone(),
        clean_inputs_for_win_saving(vid.title().to_string()),
        author,
        album.to_owned(),
        String::from("no features rn"),
        jpg.clone(),
        mp3.clone(),
    );
    match punge_obj {
        Ok(punge) => match insert::add_to_main(punge) {
            Ok(t) => Ok(t),
            Err(e) => Err(AppError::DatabaseError(e)),
        },
        Err(e) => Err(e),
    }
}

pub async fn begin_single(video: Video) -> Vec<Result<String, AppError>> {
    // can only create one item inside a vec, but done this way so the branches return the same type
    let (mp3, jpg) = fetch_json();
    // need to have the arms of the if statement handle the videos seperately since one of than can
    // create a vec of punge objects (compared to just one)
    let mut ret_vec: Vec<Result<String, AppError>> = vec![];

    // case where it is a single album upload (description check passes and title contains ' - '
    if video.title().contains(" - ")
        && description_timestamp_check(video.clone().video_details().short_description.as_str())
    {
        if Regex::new(r"\d*:\d\d")
            .unwrap()
            .is_match(video.video_details().short_description.as_str())
        {
            // need to download, pass the unique id and album name
            let (author, album) = handle_single_vid_album(&video);
            // title can be an empty string because it will be overwritten soon
            let punge_obj = create_punge_obj(
                video.to_owned(),
                "".to_string(),
                author,
                album,
                String::from("none"),
                jpg,
                mp3.clone(),
            );
            println!("messin with the objs");
            match punge_obj {
                Ok(t) => {
                    let album_songs = sep_video::seperate(
                        video.video_details().short_description.to_owned(),
                        t,
                        mp3,
                        video.video_details().length_seconds as usize,
                    );
                    for obj in album_songs {
                        match insert::add_to_main(obj) {
                            Ok(ret_val) => ret_vec.push(Ok(ret_val)),
                            Err(e) => ret_vec.push(Err(AppError::DatabaseError(e))),
                        }
                    }
                }
                Err(e) => ret_vec.push(Err(e)),
            }
        }
        ret_vec
    }
    // case of <title> - <artist> (in the title) that does not have the
    else if video.title().contains(" - ") {
        let temp = video
            .title()
            .split(" - ")
            .into_iter()
            .collect::<Vec<&str>>();
        // sometimes yt videos decide to do <title> - <artist>. and sometimes <artist> - <title>.
        let title = temp[1..].join(" - ").to_string();
        let author = temp[0].to_string();
        let album = String::from("Single");
        // do a parse for features here?
        let punge_obj = create_punge_obj(
            video,
            title,
            author,
            album,
            String::from("No features rn"),
            jpg,
            mp3,
        );
        match punge_obj {
            Ok(obj) => match insert::add_to_main(obj) {
                Ok(e) => ret_vec.push(Ok(e)),
                Err(e) => ret_vec.push(Err(AppError::DatabaseError(e))),
            },
            Err(e) => ret_vec.push(Err(e)),
        }
        ret_vec
    } else {
        // title = title, author = author, album = single, features = none
        println!("here 0");
        // in this case it is likely that title = title and author = author
        let desc = video.video_details();
        let desc = desc.short_description.split("\n").collect::<Vec<&str>>();
        // the fifth line of the description will be the album name. or single name if its a single
        let album_or_single: String = if &desc.len() < &5 {
            // edge case where description is not more than 4 lines
            String::from("Single")
        } else if &desc[4] == &video.title() {
            // if the 5th line == title, it is a single (because of "provided by: " standards)
            String::from("Single")
        } else {
            desc[4].to_string() // remember the space between the lines also count, 5th line is album!
        };
        let auth = clean_author(video.video_details().author.to_owned());
        let title = clean_inputs_for_win_saving(video.title().to_string());
        // should probably parse and find features here
        let punge_obj = create_punge_obj(
            video,
            title,
            auth,
            album_or_single,
            String::from("No features rn"),
            jpg,
            mp3,
        );
        match punge_obj {
            Ok(t) => match insert::add_to_main(t) {
                Ok(ret_val) => {
                    ret_vec.push(Ok(ret_val));
                }
                Err(e) => {
                    ret_vec.push(Err(AppError::DatabaseError(e)));
                }
            },
            Err(e) => {
                ret_vec.push(Err(e));
            }
        }
        ret_vec
    }
}

fn create_punge_obj(
    vid: Video,
    title: String,
    author: String,
    album: String,
    features: String,
    jpg_dir: String,
    mp3_dir: String,
) -> Result<PungeMusicObject, AppError> {
    // downloads the video, thumbnail
    // creates the punge obj for further processing if needed (like one song -> whole album)
    let author = clean_inputs_for_win_saving(author);
    let title = clean_inputs_for_win_saving(title);
    let album = clean_inputs_for_win_saving(album);
    let naming_conv = format!("{} - {}{}", author, title, &vid.video_details().video_id);
    let jpg_name = format!("{}{}.jpg", jpg_dir, naming_conv);
    let mp3_name = format!("{}{}.mp3", mp3_dir, naming_conv);
    if fetch::exists_in_db(vid.video_details().video_id.to_string()) {
        return Err(AppError::DatabaseError(
            DatabaseErrors::DatabaseEntryExistsError,
        ));
    }
    if std::path::Path::new(&mp3_name).exists() {
        return Err(AppError::DatabaseError(DatabaseErrors::FileExistsError));
    }
    // keep in mind that this will add to db whether it fails or not. which is intended
    download_to_punge(
        vid.clone(),
        mp3_dir,
        jpg_dir,
        mp3_name.clone(),
        jpg_name.clone(),
    )?;
    let len = sep_video::int_to_timestamp(vid.video_details().length_seconds as usize);
    Ok(PungeMusicObject {
        title,
        author,
        album,
        features,
        length: len.clone(),
        savelocationmp3: mp3_name,
        savelocationjpg: jpg_name,
        datedownloaded: chrono::Local::now().date_naive(),
        lastlistenedto: chrono::Local::now().date_naive(),
        ischild: false, // is changed if obj is passed in to be modified by sep_video.rs
        uniqueid: vid.video_details().video_id.to_string(),
        plays: 0,
        weight: 0,
        threshold: crate::db::utilities::calc_thres(len),
    })
}

fn fetch_json() -> (String, String) {
    // reason we fetch the json each time instead of having it be a static value is because when the app is open
    // the user can change the json value. So we should probably fetch it each time
    let raw_json = fs::File::open("./cache/locations.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(raw_json).unwrap();
    let mp3 = json.get("mp3_path").unwrap();
    let jpg = json.get("jpg_path").unwrap();
    let mut mp3 = mp3.as_str().unwrap().to_string();
    let mut jpg = jpg.as_str().unwrap().to_string();
    // ensure that the directories given do end with a slash of some type.
    // probably better to ensure that the user-changed input has this slash. later activity
    if !mp3.ends_with("\\") && !mp3.ends_with("/") {
        mp3.push('/')
    }
    if !jpg.ends_with("\\") && !jpg.ends_with("/") {
        jpg.push('/')
    }
    (mp3, jpg)
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

pub fn handle_single_vid_album(video: &Video) -> (String, String) {
    // returned format should be <artist> <album>. titles for songs are done elsewhere
    if video.title().contains(" - ") {
        // format is: <artist> - <album>
        let split: Vec<&str> = video.title().split(" - ").collect_vec();
        // this covers the weird case where there is multiple " - " in the title
        (split[0].to_string(), split[1..].join(" - ").to_string())
    } else {
        // we assume the format is: title = album name, artist channel of song = artist. almost all
        // instances should be covered in the if statement above
        (
            video.video_details().author.to_owned(),
            video.title().to_string(),
        )
    }
}

fn download_to_punge(
    vid: Video,
    mp3_path: String,
    _jpg_path: String,
    new_mp3_name: String,
    _new_jpg_name: String, // unused rn
) -> Result<(), AppError> {
    // let old_name = format!("{}{}.webm", mp3_path.clone(), vid.video_details().video_id);
    let mp4_name = format!("{}{}.mp4", mp3_path.clone(), vid.video_details().video_id); // can sometimes be .webm??
    let webm_name = format!("{}{}.webm", mp3_path.clone(), vid.video_details().video_id);
    println!(
        "mp4_name: {} \nwebm name: {}\ndoes path exist: {}",
        &mp4_name,
        &webm_name,
        Path::new(&mp4_name).exists()
    );
    let old_name = if Path::new(&mp4_name).exists() {
        // sometimes its an webm download, sometimes mp4. dunno why
        mp4_name
    } else {
        webm_name
    };
    // we assume that the inputs are sanitized by "clean_input_for_win_saving"
    // the unwrap can fail sometimes. so we loop 5 times, sleeping for 3 seconds inbetween so it will try again
    match vid
        .best_audio()
        .unwrap()
        .blocking_download_to_dir(mp3_path.clone())
    {
        Ok(_t) => {
            // convert the old file to (webm) to mp3 and rename
            let x = Command::new("ffmpeg.exe")
                .args([
                    "-i",
                    old_name.as_str(),
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
                    match fs::remove_file(old_name.clone()) {
                        Ok(_t) => {
                            Ok(()) // if the ffmpeg operation goes well and he file is removed
                        }
                        Err(e) => {
                            println!("nameer::  {} {}", new_mp3_name.as_str(), &old_name);
                            Err(AppError::FileError(format!("{:?}", e))) // if the ffmpeg operation works, and the file is not removed
                        }
                    }
                }
                Err(e) => {
                    Err(AppError::FfmpegError(e.to_string())) // if the ffmpeg operation fails
                }
            }
        }
        Err(e) => Err(AppError::YoutubeError(format!(
            "Error downloading {}: {:?}",
            format!(
                "https://www.youtube.com/watch?v={}",
                vid.video_details().video_id.to_string()
            ),
            e
        ))),
    }
}

fn clean_author(author: String) -> String {
    // cleans the authors name of VEVO, Official, and - topic
    let length = author.len();
    let x = if author.ends_with(" - Topic") {
        author[..length - 8].to_string()
    } else if author.ends_with("VEVO") {
        // catches both cases where vevo is either attached to the author or not
        // e.g. : KendrickVEVO | Kendrick VEVO
        let new = author[..length - 4].to_string();
        if new.ends_with(" ") {
            new[..new.len() - 1].to_string()
        } else {
            new
        }
    } else if author.ends_with("Official") {
        author[..length - 9].to_string()
    } else {
        author
    };
    x
}

fn check_for_features(title: String, author: String, description: String, ordered: bool) -> String {
    // delimeter of each feature should be a comma
    // param: ordered = is the description of the video ordered as auto-gen videos? where the features
    // are always on the same line
    if ordered {
        // gets the third line of the description, splits all of the artists by the · delimeter and joins them into a string
        // for returning
        description.split("\n").collect::<Vec<&str>>()[2]
            .split("·") // this is not a period, it is a special character
            .collect::<Vec<&str>>()
            .join(",")
    } else {
        // attempt to parse features here
        return "".to_string(); // unimplemented lol
    }
}

fn description_timestamp_check(desc: &str) -> bool {
    // answering the question: are the timestamps real?
    // this is also a bit more of an edge case function that failed on kanye "5:30", where there are lyrics in the description that include a timestamp
    // a song will have repeating "timestamps" (in lyrics), legitimate timestamps will not
    let pattern = Regex::new(r"\d*:\d\d").unwrap(); // catches timestamps (10:10, 1:34:51..)
    let mut caught_list: Vec<&str> = pattern.find_iter(desc).map(|mat| mat.as_str()).collect(); // list for all captured regex patterns. We will check if they are all the same
    if caught_list.len() == 0 {
        // if the caught list it empty, meaning that there are no timestamps
        return false;
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

fn add_to_stragglers(straggler: Straggler) {
    // un-pub at some point
    let file = fs::File::open("./cache/stragglers.json").unwrap();
    let mut json: Value = serde_json::from_reader(file).unwrap();
    json.as_array_mut().unwrap().push(json!(straggler));
    let mod_json = serde_json::to_string_pretty(&json).unwrap();
    println!("new json: {}", &mod_json);
    fs::write("./cache/stragglers.json", mod_json).unwrap();
}

fn clear_stragglers() {
    let json: Value = json!([]);
    let modded = serde_json::to_string_pretty(&json).unwrap();
    std::fs::write("./cache/stragglers.json", modded).unwrap();
}

pub fn fetch_stragglers() -> Vec<Straggler> {
    let file = fs::File::open("./cache/stragglers.json").unwrap();
    let reader = BufReader::new(file);
    let vecceroni: Vec<Straggler> = serde_json::from_reader(reader).unwrap();
    vecceroni
}

fn add_new_straggler_list(stragglers: Vec<Straggler>) {
    let json_data = json!(stragglers);
    let modded = serde_json::to_string_pretty(&json_data).unwrap();
    fs::write("./cache/stragglers.json", modded).unwrap();
}

fn fetch_straggler_count() -> usize {
    fetch_stragglers().len()
}

// fn download_stragglers() { // will come back to eventually !!
//     // will attempt to redownload all stragglers. if a straggler fails. we will write it back into stragglers.json
//     // also stragglers will / should only be singles, no playlists
//     // the data for stragglers is added into the db, but the actual download isn't there. we will download it with correct naming convention
//     let new_stragglers: Vec<Straggler> = Vec::new();
//     let ids: Vec<Straggler> = fetch_stragglers();
//     let (mp3_download, jpg_download) = fetch_json();
//     // clear stragglers. so that the download_to_punge function can put back any others that failed this time
//     clear_stragglers();
//     for straggler in ids {
//         let link = format!("www.youtube.com/watch?v={}", straggler.id);
//         let url = rustube::url::Url::parse(link.as_str()).unwrap();
//         let vid = rustube::blocking::Video::from_url(&url).unwrap();
//         download_to_punge(vid, mp3_download.clone(), jpg_download.clone(), straggler.mp3_path, straggler.jpg_path)
//     }
// }
