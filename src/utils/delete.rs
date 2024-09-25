use crate::db::fetch::get_obj_from_uuid;
use crate::db::update::delete_from_uuid;
use crate::types::AppError;
use std::fs;
use std::path::Path;

pub fn delete_record_and_file(uniqueid: &str) -> Result<(), AppError> {
    // get the filepath from the database
    let obj = get_obj_from_uuid(&uniqueid)?;
    let to_delete = Path::new(&obj.savelocationmp3);
    let res = fs::remove_file(to_delete);
    if res.is_err() {
        return Err(AppError::FileError(format!(
            "Failed deleting the file, is it there? {}",
            &obj.savelocationmp3
        )));
    };
    delete_from_uuid(uniqueid)?;
    Ok(())
}
