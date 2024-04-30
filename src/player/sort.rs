// in this file, we have the funcationality of sorting through our current playlist and find the best match for user input
// there are also 2 main sorts, one sorts author / title, the other sorts album (if album sort is choosen as best match for user input, shuffle = false, count = first song of the album)
// quite inspired by helix's regex
use crate::db::fetch::{get_all_from_playlist, get_all_main};
use crate::types::{AppError, PungeMusicObject};
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;

fn search_string(to_search: String, pattern: String) -> bool {
    let regex = Regex::new(&pattern).unwrap();
    regex.is_match(to_search.as_str())
}

fn create_new_pattern(input: String) -> String {
    let words = input.split(' ').collect::<Vec<&str>>();
    let mut patt = String::from("(?i)");
    for word in words {
        patt.push_str(format!("(.*{})", word).as_str());
    }
    patt
}

fn get_value_of_found(search_string: String, letters: String) -> u8 {
    // should probably divide total chars in search string, so you can actually find kendrick lamar - i through search
    // only does title + author rn
    let search_string = search_string.to_lowercase();
    let search_letters: Vec<char> = letters.chars().collect();
    let mut score: u8 = 0;
    let mut active_letter = 0;
    let mut streak = false;
    for letter in search_string.chars() {
        if active_letter >= search_letters.len() {
            // ignore
        } else if letter == search_letters[active_letter] {
            if streak {
                // so if we are on a streak of multiple letters, increase the value
                score += 2;
                active_letter += 1;
            } else {
                // so if this is our first letter being found (not on streak)
                score += 1;
                streak = true;
                active_letter += 1;
            }
        } else {
            // letter does not match
            // skip
            streak = false;
        }
    }
    score
}

// pub async fn get_first_match_from_db(
//     playlist: String,
//     user_string: String,
// ) -> Result<PungeMusicObject, AppError> {
//     // we actually just open a new connection to the db, and if an entry
// }

pub async fn get_values_from_db(
    playlist: String, // uniqueid now
    user_string: String,
) -> Result<PungeMusicObject, AppError> {
    let playlist_values = if playlist == *"main" {
        get_all_main().unwrap()
    } else {
        get_all_from_playlist(playlist.as_str()).unwrap()
    };
    let regex_patt = create_new_pattern(user_string.clone());
    println!("pattern: {}", &regex_patt);
    let mut found_values: Vec<(u8, PungeMusicObject)> = Vec::new();
    for music_entry in playlist_values {
        // will always be author - title
        let to_search_string = format!(
            "{} - {}",
            music_entry.author.clone(),
            music_entry.title.clone()
        );
        println!("searching: {}", &to_search_string);
        if search_string(to_search_string.clone(), regex_patt.clone()) {
            found_values.push((
                get_value_of_found(to_search_string, user_string.clone()),
                music_entry,
            ));
        }
    }
    if found_values.is_empty() {
        return Err(AppError::SearchError("Search was not found".to_string()));
    }
    found_values.sort_by_key(|item| item.0);
    Ok(found_values[0].1.clone())
}

// different shuffle modes..

// shuffle, have a bias towards songs with a higher weight
pub fn shuffle_weight_bias(grabbed: Vec<PungeMusicObject>) -> Vec<PungeMusicObject> {
    // let mut grabbed = get_all_main().unwrap();
    let mut grabbed = grabbed.clone(); // gotta be moronic
    let len = grabbed.len();
    grabbed.sort_unstable_by_key(|x| x.weight);
    let mut rng = rand::thread_rng();
    // divide the list into 6. shuffle those 6 individually, then re-insert back in place
    let mut new = vec![];
    // if total songs are less than 6... bruh. stupid edge case
    if grabbed.len() < 7 {
        grabbed
    } else {
        for chunk in grabbed.chunks_mut(len / 6) {
            chunk.shuffle(&mut rng);
            for k in chunk {
                new.push(k.to_owned())
            }
        }
        new
    }
}

// pure random shuffle, does not matter weight or anything. default choice..
pub fn regular_shuffle(grabbed: Vec<PungeMusicObject>) -> Vec<PungeMusicObject> {
    // let mut grabbed = get_all_main().unwrap();
    let mut grabbed = grabbed.clone(); // chat is this stupid ?
    let mut rng = rand::thread_rng();
    grabbed.shuffle(&mut rng);
    grabbed
}

// cant be used since the player tries to grab the location of the uuid in the list,
// but this method does not guarentee that there will be that value
// stupid, i would never personally use it, can cause duplicate songs.. let the user choose tho
pub fn true_random_shuffle(grabbed: Vec<PungeMusicObject>) -> Vec<PungeMusicObject> {
    // let grabbed = get_all_main().unwrap();
    // let mut rng = rand::thread_rng();
    // let mut new = vec![];
    // for _ in grabbed.iter() {
    //     new.push(grabbed[rng.gen_range(0..grabbed.len())].clone())
    // }
    // new
    // temp replacement, or maybe forever idk
    let mut grabbed = grabbed.clone(); // chat is this stupid ?
    let mut rng = rand::thread_rng();
    grabbed.shuffle(&mut rng);
    grabbed
}

pub fn cluster_shuffle(grabbed: Vec<PungeMusicObject>) -> Vec<PungeMusicObject> {
    // sort of like weighted shuffle, but the 'weight' in this case is just based off the order
    // this is literally just taken from that, nevermind the ordering on weight part.
    let mut grabbed = grabbed.clone();
    let len = grabbed.len();
    let mut rng = rand::thread_rng();
    let mut new = vec![];
    if grabbed.len() < 7 {
        grabbed
    } else {
        for chunk in grabbed.chunks_mut(len / 6) {
            chunk.shuffle(&mut rng);
            for k in chunk {
                new.push(k.to_owned())
            }
        }
        new
    }
}
