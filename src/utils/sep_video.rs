// this file is called from /.decide_youtube.rs and serves to seperate a webm video into multiple parts
// if said video is an album upload. Uses ffmpeg_rust to slice videos
use crate::playliststructs::PungeMusicObject;
use chrono::Local;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub fn seperate(
    description: String,
    obj: PungeMusicObject,
    mp3_dir: String,
    length: usize,
) -> Vec<PungeMusicObject> {
    // vec of paths to files
    // path: path to the modified mp3 file. the files will be placed in the dir, and this file removed
    // timestamps: timestamps in <seconds song begins at, title of song>
    // dir: directory that the mp3 sits in. We could derive it from obj.savelocationmp3, but that would be a lot of work (split by / or \\)?
    let mut ret_vec: Vec<PungeMusicObject> = Vec::new();
    let map = generate_map(description);
    let timestamp_map = hash_to_vec_ordered(map, length);
    let length_map = get_length_from_timestamp(timestamp_map.clone());
    println!("Seperating!!. {}", &obj.savelocationmp3);
    for (count, (start_time, end_time, title)) in timestamp_map.iter().enumerate() {
        let out_path = format!(
            "{}{} - {}{}.mp3",
            mp3_dir, &obj.author, &title, &obj.uniqueid
        );
        println!("exporting to {}", &out_path);
        let args = if end_time != "end" {
            vec![
                "-i",
                &obj.savelocationmp3.as_str(),
                "-ss",
                start_time.as_str(),
                out_path.as_str(),
            ]
        } else {
            println!(
                "hit the else?: {}\n{}\n{}\n",
                &obj.savelocationmp3, &start_time, &out_path
            );
            vec![
                "-i",
                &obj.savelocationmp3.as_str(),
                "-ss",
                start_time.as_str(),
                out_path.as_str(),
            ]
        };
        let cmd = Command::new("ffmpeg.exe").args(args).output();
        println!("{:?}", cmd.unwrap());
        let new_obj = PungeMusicObject {
            title: title.to_owned(),
            author: obj.author.to_owned(),
            album: obj.album.to_owned(),
            features: "none".to_string(),
            length: length_map[count].clone(),
            savelocationmp3: out_path.clone(),
            savelocationjpg: obj.savelocationjpg.to_owned(),
            datedownloaded: Local::now().date_naive(),
            lastlistenedto: Local::now().date_naive(),
            ischild: true,
            uniqueid: format!("{}{}", obj.uniqueid, count.to_string()),
            plays: 0,
            weight: 0,
            threshold: crate::db::utilities::calc_thres(length_map[count].clone()),
        };
        ret_vec.push(new_obj)
    }
    // remove orignal 1 upload file
    fs::remove_file(obj.savelocationmp3).unwrap();
    ret_vec
}

pub fn hash_to_vec_ordered(
    map: HashMap<String, String>,
    total_len: usize,
) -> Vec<(String, String, String)> {
    // point of this function is to turn the start time and name into start time, end time, title, length
    let mut key_order: Vec<&String> = map.keys().collect();
    key_order.sort();
    let mut ret_vec: Vec<(String, String, String)> = Vec::new();
    for key in map.keys() {
        if let Some(ind) = key_order.iter().position(|&x| x == key) {
            // get the index of given key inside of the sorted key list
            let start_time = key_order[ind];
            // last song will take the 'total_len' as length. usize -> 'hh:mm:ss'
            let end_time = if ind + 1 >= map.len() {
                int_to_timestamp(total_len)
            } else {
                key_order[ind + 1].to_string()
            };
            // (start_time, end_time, title)
            let vec_vals: (String, String, String) =
                (start_time.to_owned(), end_time, map[key].clone());
            ret_vec.push(vec_vals)
        } else {
            panic!("Key not found in key list? Should be impossible")
        }
    }
    // sort the tuples inside of the vec by the first element !
    ret_vec.sort_by(|a, b| a.0.cmp(&b.0));
    ret_vec
}
// pass in the length of the song in "hh:mm:ss" format
pub fn generate_map(description: String) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let split_by_new_line: Vec<&str> = description.split("\n").collect();
    let timestamp_reg = Regex::new(r#"\d+:\d+"#).unwrap();
    for line in split_by_new_line {
        let timestamper = Regex::find(&timestamp_reg, line);
        match timestamper {
            Some(t) => {
                let title = Regex::new(r#"[a-zA-Z].*[a-zA-Z]"#).unwrap();
                let matches: Vec<&str> = title.find_iter(line).map(|t| t.as_str()).collect();
                let title_string = matches.join(" ");
                let new_timestamp = fix_timestamp(t.as_str());
                map.insert(new_timestamp, title_string);
            }
            None => continue,
        }
    }
    map
}

fn fix_timestamp(timestamp: &str) -> String {
    // adding 00: if there is not a hh portion of hh:mm:ss
    let mut new_stamp = String::new();
    if timestamp.len() != 8 {
        new_stamp += "00:"
    }
    new_stamp += timestamp;
    new_stamp
}

// pass in the two timestamps (starts at, ends at, and title). title is just not used
pub fn get_length_from_timestamp(timestamp: Vec<(String, String, String)>) -> Vec<String> {
    let mut total_length_seconds = 0; // total seconds of the song. used when the final end time is unknown (last song)
    let mut length_vec: Vec<String> = Vec::new();
    for (start, end, _title) in timestamp {
        let start = timestamp_to_int(start);
        let end = if end == "end" {
            total_length_seconds + start.clone()
        } else {
            timestamp_to_int(end)
        };
        let val = end - start;
        total_length_seconds += val.clone();
        let ret_val = int_to_timestamp(val);
        length_vec.push(ret_val)
    }
    length_vec
}

fn timestamp_to_int(timestamp: String) -> usize {
    let mut val = 0;
    let bruh: Vec<String> = timestamp
        .split(":")
        .collect_vec()
        .iter()
        .map(|item| item.to_string())
        .collect();
    let (mut hour, mut minute, second): (usize, usize, usize) = (
        bruh[0].parse().unwrap(),
        bruh[1].parse().unwrap(),
        bruh[2].parse().unwrap(),
    );
    while hour != 0 {
        hour -= 1;
        val += 3600
    }
    while minute != 0 {
        minute -= 1;
        val += 60
    }
    val += second;
    val
}

pub fn int_to_timestamp(mut seconds: usize) -> String {
    // used in decide_youtube as well
    let mut hrs = 0;
    let mut minutes = 0;
    while seconds >= 3600 {
        hrs += 1;
        seconds -= 3600;
    }
    while seconds >= 60 {
        minutes += 1;
        seconds -= 60;
    }
    let hrs_str = hrs.to_string();
    let min_str = minutes.to_string();
    let sec_str = seconds.to_string();
    let mut new_time: String = String::new();
    for mut string in vec![hrs_str, min_str, sec_str] {
        if string.len() != 2 {
            string = format!("{}{}", "0", string);
        }
        new_time += string.as_str();
        new_time.push(':')
    }
    new_time.pop(); // remove the last ':' that is added
    new_time
}
