// in this file, we are adding a new field to our db "threshold", which is calculated from existing values
// the purpose is for our src\gui\subscription.rs database subscription to read a value from db (instead of doing an unneeded calculation to get it)

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
