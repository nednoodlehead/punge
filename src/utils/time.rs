// this is a file for converting time types for use in the player, and the resulting player
// also, this commit should change the data type of "length" in "main" table

pub fn sec_to_time(int: std::time::Duration) -> String {
    // format: HH:MM:SS
    // i do not claim to be good at writing these "algo" type functions.
    if int < std::time::Duration::from_secs(1) {
        return String::from("0:00");
    }
    let int = int.as_secs();
    let hours = int / 3600; // how many hours are in our seconds
    let no_hrs = int - (hours * 3600); // need to remove the hours...
    let minutes = no_hrs / 60;
    let seconds = no_hrs % 60;
    let hour_str = if hours == 0 {
        String::new()
    } else {
        format!("{:02}:", hours)
    };
    let min_str = if hours > 0 && minutes < 10 {
        format!("{:02}", minutes)
    } else {
        minutes.to_string()
    };
    format!("{}{}:{:02}", hour_str, min_str, seconds)
}

pub fn time_to_sec(time: &str) -> u32 {
    // should all be HH:MM:SS
    let times: Vec<&str> = time.split(':').collect();
    let mut val: u32 = 0;
    for (count, x) in times.iter().enumerate() {
        let num = x.parse::<u32>().unwrap();
        if count == 0 {
            val += num * 3600 // hour
        }
        if count == 1 {
            val += num * 60 // minute
        }
        if count == 2 {
            val += num; // second :D
        }
    }
    val
}

pub fn total_time_conv(og: &str) -> String {
    let int: usize = og.parse().unwrap();
    let hours = int / 3600; // how many hours are in our seconds
    let no_hrs = int - (hours * 3600); // need to remove the hours...
    let minutes = no_hrs / 60;
    let seconds = no_hrs % 60;
    let hour_str = if hours == 0 {
        String::new()
    } else {
        format!("{:02} hours ", hours)
    };
    let min_str = if hours > 0 && minutes < 10 {
        format!("{:02} minutes ", minutes)
    } else {
        format!("{} minutes ", minutes)
    };
    format!("{}{}{:02} seconds", hour_str, min_str, seconds)
}
