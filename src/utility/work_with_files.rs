use std::fs::File;
use std::io::Write;

use rustc_serialize::json::Json;

///
///
/// # Arguments
///
/// * `file_name`: имя файла
///
/// returns: File
///
/// # Examples
///
pub fn create_file(file_name: String) -> File {
    return File::create(file_name).expect("create failed");
}

///
///
/// # Arguments
///
/// * `file`: файл в который надо записать данные
/// * `name`: имя сервиса
/// * `json`: сам объект для записи
///
/// returns: ()
///
pub fn write_file(mut file: File, name: String, json: &Json) {
    file.write_all(name.as_ref()).expect("write failed: service name");
    file.write_all("\n".as_ref()).expect("write failed: line translation");
    file.write_all(json.to_string().as_bytes()).expect("write failed json");
}