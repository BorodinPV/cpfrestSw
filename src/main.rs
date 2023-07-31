extern crate serde;


use std::collections::HashMap;
use std::io::Write;

use clap::Parser;
use rustc_serialize::json::Json;

use crate::utility::{utils, work_with_files};

pub mod utility;


fn main() {
    let mut map: HashMap<String, &Json> = HashMap::new();
    let response = utils::get_info_for_service(String::from("http://localhost:8383/s3client/v3/api-docs"));

    let component = utils::get_json_component("paths".parse().unwrap(), &response);
    map.insert("service".to_string(), component);
    println!("Значение: {:?}", map.get("service"));

    let file = work_with_files::create_file("s3client".to_string());
    work_with_files::write_file(file,"s3client".to_string(), component);

    println!("data written to file");
}