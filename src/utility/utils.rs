use rustc_serialize::json::Json;

///
///
/// # Arguments
///
/// * `url`: адрес на который нужно сделать запрос.
///
/// returns: Json
///
pub fn get_info_for_service(url: String) -> Json {
    let resp = reqwest::blocking::get(url)
        .expect("REASON");
    return Json::from_str(&*resp.text().unwrap()).unwrap();
}

///
///
/// # Arguments
///
/// * `str`: компонент который нужно получить.
/// * `obj`: объект из котрого нужно получить компонент.
///
/// returns: &Json
///
pub fn get_json_component(str: String, obj: &Json) -> &Json {
    let data = obj.as_object().unwrap();
    return data.get(&*str).unwrap();
}