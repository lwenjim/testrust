use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    // 打印时间戳和当前系统时间
    let date = SystemTime::now();
    let time = date.duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("{}", time);
    println!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    // 打印时间
    let custom_time = NaiveTime::parse_from_str("10:27:00", "%H:%M:%S").unwrap();
    println!("{}", custom_time);

    // 打印日期
    let custom_date = NaiveDate::parse_from_str("2024-12-27", "%Y-%m-%d").unwrap();
    println!("{}", custom_date);

    // 打印日期时间
    let custom_date_time =
        NaiveDateTime::parse_from_str("2024-12-27 10:27:00", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("{}", custom_date_time);

    let result = eval::eval("3/1.3").unwrap().as_f64().unwrap();
    println!("{:.2?}", result);
}
