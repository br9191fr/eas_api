#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
lazy_static! {
    pub static ref LOCATIONS: Mutex<HashMap<i32, &'static str>> =
    Mutex::new(generate_static_locations());
}
pub fn generate_static_locations() -> HashMap<i32, &'static str> {
    let mut m = HashMap::new();
    m.insert(0, "data0");
    m
}
pub fn build_static_locations(w: i32, file_to_archive: &String) -> i32 {
    let ad_where = w;
    let mut locations = LOCATIONS.lock().unwrap();
    locations.insert(ad_where, string_to_static_str(file_to_archive.to_string()));
    return ad_where;
}