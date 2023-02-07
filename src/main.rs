extern crate chrono;

use chrono::Local;
use chrono::Datelike;

fn main() {
    let now = Local::now();
    let week: String = week_str(now.weekday().number_from_monday());
    println!("📅 {} {}", week, now.format("%d/%m %H:%M:%S"));
}

fn week_str(n: u32) -> String {
    let week = match n {
        0 => "dom",
        1 => "seg",
        2 => "ter",
        3 => "qua",
        4 => "qui",
        5 => "sex",
        6 => "sáb",
        _ => "dom",
    };
    week.to_string()
}
