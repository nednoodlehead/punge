// in this file, we have the funcationality of sorting through our current playlist and find the best match for user input
// there are also 2 main sorts, one sorts author / title, the other sorts album (if album sort is choosen as best match for user input, shuffle = false, count = first song of the album)
// quite inspired by helix's regex
use crate::db::fetch::{get_all_from_playlist, get_all_main, get_uuid_from_name};
use crate::types::PungeMusicObject;
use regex::Regex;

fn search_string(to_search: String, pattern: String) -> bool {
    let regex = Regex::new(&pattern).unwrap();
    regex.is_match(to_search.as_str())
}

fn create_alt_pattern(input: String) -> String {
    // if works,
    let escaped_regex = regex::escape(input.as_str());
    let mut pattern = String::from(r"(?i)");
    for letter in escaped_regex.chars() {
        pattern.push_str(&format!(".*{}.*", letter));
    }
    pattern
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
        } else {
            if letter == search_letters[active_letter] {
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
    }
    score
}

pub fn get_values_from_db(playlist: String, user_string: String) -> Vec<(u8, PungeMusicObject)> {
    let playlist_values = if playlist == "main".to_string() {
        get_all_main().unwrap()
    } else {
        get_all_from_playlist(get_uuid_from_name(playlist).as_str()).unwrap()
    };
    let regex_patt = create_alt_pattern(user_string.clone());
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
    found_values.sort_by_key(|item| item.0);
    found_values
}
