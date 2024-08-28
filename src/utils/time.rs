// this is a file for converting time types for use in the player, and the resulting player
// also, this commit should change the data type of "length" in "main" table

pub fn sec_to_time(mut int: u32) -> String {
    // format: HH:MM:SS
    // thanks ai, i literally forget about the usefulnes of the modulus operator everday
    let hours = int / 3600; // how many hours are in our seconds
    int = int - (hours * 3600); // need to remove the hours...
    let minutes = int / 60;
    let seconds = int % 60;
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
