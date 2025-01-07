// in this file, we are adding a new field to our db "threshold", which is calculated from existing values
// the purpose is for our src\gui\subscription.rs database subscription to read a value from db (instead of doing an unneeded calculation to get it)
use crate::types::AppError;
use crate::types::PungeMusicObject;
use rusqlite::Connection;
// old, keep around for now..
pub fn _calc_thres(len: String) -> u16 {
    // len: 00:10:12 format
    let sep: Vec<&str> = len.split(':').collect();
    // nothing reaches this in the db lol let hrs = sep[0].parse::<u16>().unwrap();
    let min = sep[1].parse::<u16>().unwrap();
    let sec = sep[2].parse::<u16>().unwrap();
    let total = (min * 60) + sec;
    if total / 15 == 0 {
        // so if the song is sub 15 seconds, we have the case for that
        0
    } else {
        (total / 15) - 1 // purpose is for punge to be able to detect (using gui\subscription::databasesub_2 rn name) in the case where the first 15 seconds is not detected because of the random
                         // 15 second timer, and if it started right at the end of the first song
    }
}

pub fn calc_thres(len: usize) -> usize {
    if len / 15 == 0 {
        0
    } else {
        (len / 15) - 1
    }
}

pub fn validate_song_nums() -> Result<(), AppError> {
    crate::utils::backup::create_backup("./".to_string()).unwrap();
    let main_list = crate::db::fetch::get_all_main().unwrap();
    let mut new_list: Vec<PungeMusicObject> = vec![];
    let mut num_list: Vec<usize> = vec![];
    let mut largest_num: usize = 0;
    for item in main_list.into_iter() {
        if num_list.contains(&item.order) {
            // the db has an item with this number already.
            let mut new_item = item.clone();
            new_item.order = largest_num + 1;
            println!(
                "{} found. ({} - {}). Setting count={}",
                item.order, &item.title, &item.author, new_item.order
            );
            num_list.push(new_item.order);
            new_list.push(new_item);
            largest_num += 1;
        } else {
            // set the largest number to... the largest number.
            // only checked here because if the number is in the num_list, it has already been checked as te largest number
            if item.order > largest_num {
                largest_num = item.order;
            }
            num_list.push(item.order);
            new_list.push(item);
        }
    }
    std::fs::remove_file("main.db").unwrap();
    crate::db::create_db::create_table_defaults().unwrap();
    crate::db::insert::add_to_main_bulk(new_list).unwrap();
    Ok(())
}

pub fn fix_song_count_gaps() -> Result<(), AppError> {
    let mut what_num_should_be = 0;
    let main_list = crate::db::fetch::get_all_main().unwrap();
    let mut fixed_list: Vec<PungeMusicObject> = vec![];
    for item in main_list.into_iter() {
        let mut cloned_item = item.clone();
        cloned_item.order = what_num_should_be;
        fixed_list.push(cloned_item);
        what_num_should_be += 1;
    }
    for x in fixed_list {
        println!("{} - {} ({})", x.title, x.author, x.order);
    }
    Ok(())
}
