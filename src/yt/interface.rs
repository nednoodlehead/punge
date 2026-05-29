use crate::db::insert::add_to_main;
use crate::gui::messages::ProgramCommands;
use crate::types::{AppError, DatabaseErrors, PungeMusicObject, YouTubeData};
use crate::utils::sep_video;
use itertools::Itertools;
use log::{debug, error, info, warn};
use regex::Regex;
use rusqlite;
use rusty_ytdl::Video;
use sipper::{sipper, Sipper, Straw};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
// it is assumed that the link passed in here should be checked for it being a playlist.
// so every link should be just downloading one video
// we do need to know if this function was called under the pretext of us knowing the album / playlist title

pub async fn playlist_wrapper(
    link: String,
) -> Result<rusty_ytdl::search::Playlist, rusty_ytdl::VideoError> {
    rusty_ytdl::search::Playlist::get(link, None).await
}

pub fn download_vid_to_punge<'a>(
    url: String,
    playlist_title: Option<&'a str>,
    order: usize,
) -> impl Straw<PungeMusicObject, ProgramCommands, AppError> + use<'a> {
    // get info -> check if exists -> download -> stream download to main thread using sipper -> once done insert into main
    sipper(move |mut sender| async move {
        // define some video options
        let vid_opt = rusty_ytdl::VideoOptions {
            quality: rusty_ytdl::VideoQuality::LowestVideo,
            filter: rusty_ytdl::VideoSearchOptions::Audio,
            download_options: rusty_ytdl::DownloadOptions::default(),
            request_options: rusty_ytdl::RequestOptions::default(),
        };
        let video = Video::new_with_options(&url, vid_opt)
            .map_err(|e| AppError::YoutubeError(e.to_string()))?; // url check
        let video_id = video.get_video_id();
        let video_details = video.get_basic_info().await.unwrap().video_details;
        let (mp3, jpg) = fetch_json();
        let jpg_file = format!("{}{}.jpg", &jpg, &video_id);
        let final_details = decipher_details(video_details.clone(), playlist_title)
            .await
            .unwrap(); // pretty sure it cannot fail
        let author = clean_inputs_for_win_saving(clean_author(final_details.author));
        let title = clean_inputs_for_win_saving(clean_title(final_details.title));
        let album = clean_inputs_for_win_saving(final_details.album);
        // where the song will be saved...
        let naming_conv = format!("{} - {}{}", author, title, video_id.clone());
        info!("the file will be named {}", &naming_conv);
        let mp3_file = format!("{}{}.mp3", mp3, naming_conv);
        let punge_obj = create_punge_music_obj(
            title,
            author,
            album,
            "none".to_string(),
            mp3_file.clone(),
            jpg_file.clone(),
            video_id.clone(),
            video_details
                .length_seconds
                .parse()
                .expect("Somehow not a number?"),
            order,
        );
        if check_if_exists(&video_id) && playlist_title.is_none() {
            // if the entry exists already
            warn!("The video already exists");
            return Err(AppError::DatabaseError(
                DatabaseErrors::DatabaseEntryExistsError,
            ));
        }
        if std::path::Path::new(&mp3_file).exists() {
            // should this be checked for eariler? can we???
            return Err(AppError::DatabaseError(DatabaseErrors::FileExistsError));
        }
        let result = init_download(&url, &mp3_file, &jpg_file, &video_id)
            .run(&sender)
            .await;
        match result {
            Ok(()) => return Ok(punge_obj),
            Err(e) => return Err(e),
        }
    })
}

// pub fn download_playlist_to_punge(
//     link: String,
// ) -> impl Straw<Vec<Result<YouTubeData, AppError>>, ProgramCommands, AppError> {
//     // we download an entire playlist here, called straight from the task::sip in ProgramCommands::Download
//     sipper(move |mut sender| async move {
//         let playlist = rusty_ytdl::search::Playlist::get(link, None).await.unwrap();
//         for video in playlist.videos {
//             let details = dicipher_playlist_details(&video, &playlist);
//         }
//     });
//     unimplemented!()
// }

pub async fn dicipher_playlist_details(
    video: &rusty_ytdl::search::Video,
    playlist: &rusty_ytdl::search::Playlist,
) -> YouTubeData {
    if playlist.name.contains(" - ") {
        // playlist name contains a dash, meaning it is in the format of author - album
        let playlist_split = playlist.name.split(" - ").collect::<Vec<&str>>();
        let (author, album) = (playlist_split[0], playlist_split[1]);
        // weird situation where a fan makes an album, and the artist has "Name - Title" in the titles...
        if video.title.contains(" - ") {
            let title = video.title.split(" - ").collect::<Vec<&str>>()[0];
            return YouTubeData::new(title, author, album);
        }
        return YouTubeData::new(video.title.clone(), author, album);
    } else if video.description.starts_with("Provided") {
        // playlist title is assumed to be album if no dash is present
        let title = video.title.clone();
        let author = video.channel.name.clone(); // not possible to fail ... ???
        let album = video.description.split('\n').collect_vec()[4].to_string();
        return YouTubeData::new(title, author, album);
    } else {
        // not quite sure, we're going to assume that title = title, author = author, playlist title = album
        return YouTubeData::new(
            video.title.clone(),
            video.channel.name.clone(),
            playlist.name.clone(),
        );
    }
}

pub async fn decipher_details(
    details: rusty_ytdl::VideoDetails,
    playlist_title: Option<&str>,
) -> Result<YouTubeData, AppError> {
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
        };
        // let obj = create_punge_obj(
        //     video,
        //     yt_data.clone(),
        //     String::from("None"),
        //     jpg,
        //     mp3,
        //     jpg_file,
        //     details.video_id,
        //     details.length_seconds.parse::<u32>().unwrap(),
        //     order,
        // )
        // .await?;
        // add_to_main(obj)?;
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
            album: album.to_string(),
        };
        // let obj = create_punge_obj(
        //     video,
        //     youtube_data.clone(),
        //     String::from("None"),
        //     jpg,
        //     mp3,
        //     jpg_file,
        //     details.video_id,
        //     details.length_seconds.parse::<u32>().unwrap(),
        //     order,
        // )
        // .await?;
        // info!("updating: {}", &obj.title);
        // add_to_main(obj)?;
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
        };
        // SPECIAL CASE TODO
        // let temp_punge_obj = create_punge_obj(
        //     video.clone(),
        //     yt_data.clone(),
        //     String::from("none"),
        //     jpg.clone(),
        //     mp3.clone(),
        //     jpg_file,
        //     details.video_id.clone(),
        //     details.length_seconds.parse::<u32>().unwrap(),
        //     order,
        // );
        // let punge_iter = sep_video::separate(
        //     details.description,
        //     temp_punge_obj.await.unwrap(),
        //     mp3.clone(),
        //     details.length_seconds.parse::<usize>().unwrap(),
        //     order,
        // );
        // for sub_item in punge_iter {
        //     add_to_main(sub_item)?;
        // }
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
        };
        // let obj = create_punge_obj(
        //     video,
        //     youtube_data.clone(),
        //     String::from("None"),
        //     jpg,
        //     mp3,
        //     jpg_file,
        //     details.video_id,
        //     details.length_seconds.parse::<u32>().unwrap(),
        //     order,
        // )
        // .await?;
        // add_to_main(obj.clone())?;
        youtube_data
    } else {
        // these if elifs cannot find any recognized format. default to this...
        let youtube_data = YouTubeData {
            title: details.title,
            author: details.author.unwrap().name,
            album: String::from("Single"),
        };
        // let obj = create_punge_obj(
        //     video,
        //     youtube_data.clone(),
        //     String::from("None"),
        //     jpg,
        //     mp3,
        //     jpg_file,
        //     details.video_id,
        //     details.length_seconds.parse::<u32>().unwrap(),
        //     order,
        // )
        // .await?;
        // add_to_main(obj.clone())?;
        youtube_data
    };
    Ok(youtube_data)
}

pub fn check_if_exists(uniqueid: &str) -> bool {
    // maybe should be Result!?
    // checks if the given unique id is found inside the main table. aka: has it been downloaded?
    let conn = rusqlite::Connection::open("main.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM main WHERE uniqueid = ?")
        .unwrap();
    let exists = stmt.exists([uniqueid]).unwrap();
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

pub fn create_punge_music_obj(
    title: String,
    author: String,
    album: String,
    features: String,
    mp3_file: String,
    jpg_file: String,
    vid_id: String,
    vid_length: u32, // pass in this and vid_id to avoid calling .await unnecessarily
    order: usize,    // tells us where in 'main' the object sits
) -> PungeMusicObject {
    // downloads the video, thumbnail
    // creates the punge obj for further processing if needed (like one song -> whole album)
    // i am also choosing to have the naming conventions for the jpg & mp3 files to be different, for a few reasons:
    // 1. if jpg becomes similar to the "author - title" variation, we do not know at the time of the jpg download
    // (only if it is a temp file) what the author - title actually is. so we cannot just move it over
    // 2. if the mp3 becomes like the jpg file (uniqueid.jpg) then debugging the audio (and what has downloaded) is much harder
    // and it also makes moving the files around platforms much easier
    // keep in mind that this will add to db whether it fails or not. which is intended
    // maybe we should have the id & link passed around a bit more.. properly?
    let obj = PungeMusicObject {
        title,
        author,
        album,
        features,
        length: vid_length,
        savelocationmp3: mp3_file,
        savelocationjpg: jpg_file,
        datedownloaded: chrono::Local::now().date_naive(),
        lastlistenedto: chrono::Local::now().date_naive(),
        ischild: false, // is changed if obj is passed in to be modified by sep_video.rs
        uniqueid: vid_id,
        plays: 0,
        weight: 0,
        threshold: crate::db::utilities::calc_thres(vid_length as usize) as u16,
        order,
    };
    obj
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

pub fn init_download<'a>(
    link: &str,
    output_path: &'a str,
    jpg_path: &'a str,
    id: &str,
) -> impl Straw<(), ProgramCommands, AppError> + use<'a> {
    let link = link.to_string();
    let id = id.to_string();
    println!("link: {} {} {} {}", &link, &output_path, &jpg_path, &id);
    sipper(move |mut sender| async move {
        let temp_path = format!("./{}", id); // exports as .opus, so ./<id>.opus
        let mut cmd = Command::new("yt-dlp.exe")
            .args([
                "-x",
                &link,
                "-o",
                &temp_path,
                "--write-thumbnail",
                "--convert-thumbnails",
                "jpg",
                "--newline",
                "--progress",
            ])
            .stdout(Stdio::piped()) // required or stdout is None
            .spawn()
            .map_err(|e| AppError::YoutubeError(e.to_string()))
            .unwrap();

        if let Some(stdout) = cmd.stdout.take() {
            let mut line = BufReader::new(stdout).lines();
            while let Some(line) = line.next_line().await.unwrap() {
                sender
                    .send(ProgramCommands::YouTubeDownloadProgress(line))
                    .await;
            }
        }

        let status = cmd
            .wait()
            .await
            .map_err(|e| AppError::YoutubeError(e.to_string()))
            .unwrap();

        if status.success() {
            // it ran successfully, now we can convert to mp3
            sender
                .send(ProgramCommands::YouTubeDownloadProgress("50".to_string())) // meaning we are 50% done cause we've downloaded it!
                .await;
            let mut ffmpeg_cmd = Command::new("ffmpeg.exe")
                .args([
                    "-i",
                    &format!("{}.opus", &temp_path),
                    "-vn",
                    "-c:a",
                    "libmp3lame",
                    "-b:a",
                    "192K",
                    &output_path,
                ])
                .stdout(Stdio::piped())
                .spawn()
                .map_err(|e| AppError::FfmpegError(e.to_string()))
                .unwrap();
            if let Some(stdout) = ffmpeg_cmd.stdout.take() {
                let mut line = BufReader::new(stdout).lines();
                while let Some(line) = line
                    .next_line()
                    .await
                    .map_err(|e| AppError::FfmpegError(e.to_string()))
                    .unwrap()
                {
                    sender
                        .send(ProgramCommands::YouTubeDownloadProgress(line))
                        .await;
                }
            }
            // copy the jpg path to the correct location
            match std::fs::copy(format!("./{}.jpg", &id), jpg_path) {
                Ok(t) => Ok(()),
                Err(e) => Err(AppError::FileError(e.to_string())),
            }
        } else {
            Err(AppError::YoutubeError("Unable to download...".to_string()))
        }
    })
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

fn clean_title(title: String) -> String {
    // tries to clean the slop from the title. anything in brackets gets removed
    let mut new_word = String::from("");
    let mut inside_brackets: bool = false;
    for letter in title.chars() {
        if inside_brackets {
            if ['}', ']', '}'].contains(&letter) {
                inside_brackets = false;
            }
        }
        if ['[', '{', '('].contains(&letter) {
            inside_brackets = true;
        } else {
            if !inside_brackets {
                new_word.push(letter);
            }
        }
    }

    return new_word.trim_end().to_owned();
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

pub async fn insert_obj_and_cleanup(obj: PungeMusicObject) -> Result<(), AppError> {
    let id = &obj.uniqueid;
    let jpg_path = format!("./{}.jpg", &id);
    let opus_path = format!("./{}.opus", &id);
    let db_add = add_to_main(obj);
    match db_add {
        Ok(t) => {
            // we can remove the files from the root of the directory
            for path in [jpg_path, opus_path].iter() {
                let path_try_remove = std::fs::remove_file(path);
                match path_try_remove {
                    Ok(t) => {
                        info!("old file successfully removed: {}", &path)
                    }
                    Err(e) => {
                        warn!("old path unable to be removed: {}", &path)
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            warn!("Error adding into main");
            return Err(AppError::DatabaseError(DatabaseErrors::Other(
                e.to_string(),
            )));
        }
    }
}
