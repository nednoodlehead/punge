pub fn format_views(views: u64) -> String {
    // im not really the goat at algorithms. hopefully this isn'y super garbage
    let str_ver = views.to_string();
    let mut sub_count = 0;
    let mut new_str = String::new();
    for num in str_ver.chars().rev().into_iter() {
        if sub_count == 3 {
            new_str.push(',');
            sub_count = 0;
        }
        new_str.push(num);
        sub_count += 1;
    }
    new_str.chars().rev().collect()
}
