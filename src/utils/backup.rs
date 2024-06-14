use chrono::offset::Local;
use log::info;
use std::fs::copy;
pub fn create_backup(mut backup_dir: String) -> Result<(), std::io::Error> {
    // maybe backup_dir should derive from config? no big deal for now though.
    backup_dir = if !backup_dir.ends_with('/') | !backup_dir.ends_with('\\') {
        format!("{}/", backup_dir)
    } else {
        backup_dir
    };
    info!("backup location: {}", &backup_dir);
    // output should look like: c:/my_backup/punge_backup_2024-01-29-hh-mm-ss.db
    let output_path = format!(
        "{}punge_backup{}.db",
        backup_dir,
        Local::now().to_string()[..19].replace(':', "-") // get only the important part we want :)
    );
    copy(String::from("./main.db"), output_path)?;

    Ok(())
}
