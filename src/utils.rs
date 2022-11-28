#![allow(dead_code)]
use std::io::Write;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use lazy_static::lazy_static;

use chrono::prelude::*;
use env_logger::fmt::Formatter;
use env_logger::Builder;
use log::{LevelFilter, Record};

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
    println!("F to archive is {}",file_to_archive);
    ad_where
}

pub fn setup_logger(log_thread: bool, rust_log: Option<&str>) {
    let output_format = move |formatter: &mut Formatter, record: &Record| {
        let thread_name = if log_thread {
            format!("(t: {}) ", thread::current().name().unwrap_or("unknown"))
        } else {
            "".to_string()
        };

        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        writeln!(
            formatter,
            "{} {}{} - {} - {}",
            time_str,
            thread_name,
            record.level(),
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter(None, LevelFilter::Info);

    rust_log.map(|conf| builder.parse_filters(conf));

    builder.init();
}
