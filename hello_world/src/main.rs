use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    // 打印时间戳和当前系统时间
    let date = SystemTime::now();
    let time = date.duration_since(UNIX_EPOCH).unwrap().as_secs();
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

    // 执行表达式
    let result = eval::eval("3/1.3").unwrap().as_f64().unwrap();
    println!("{:.2?}", result);

    // 数据类型转换
    // 数字和字符串之间的转换
    let age = 18;
    let string_age = age.to_string();
    let back_int = string_age.parse::<i32>().unwrap();
    let back_uint = string_age.parse::<u32>().unwrap();
    println!(
        "{age}, {string_age}, {back_int}, {back_uint}",
        age = age,
        string_age = string_age,
        back_int = back_int,
        back_uint = back_uint
    );

    // 字符串和bool类型之间的转换
    let is_open = "true";
    let is_open_bool = is_open.parse::<bool>().unwrap();
    println!(
        "{is_open_bool}, {is_open}",
        is_open_bool = is_open_bool,
        is_open = is_open
    );

    // 数字之间的转换
    let num: i32 = 123;
    let num_u32: u32 = num as u32;
    let num_min = 128i16 as i8;
    println!("{}, {}, {}", num, num_u32, num_min);

    // 引用与解引用
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    fn calculate_length(s: &str) -> usize {
        s.len()
    }
    fn change(s: &mut String) {
        s.push_str(", world");
    }
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
    change(&mut s1);
    println!("{}", s1);

    // 悬垂引用
    fn dangle() -> String {
        String::from("hello")
    }
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // 操作utf-8 字符串
    for c1 in "中国人".chars() {
        println!("{}", c1);
    }
    for c2 in "中国人".bytes() {
        println!("{}", c2);
    }

    // 元组
    let tup = (1, 2, 3);
    println!("{}", tup.0);

    // 结构体
    #[derive(Clone)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let user1 = User {
        active: true,
        username: "lwj".to_string(),
        email: "1313123@qq.com".to_string(),
        sign_in_count: 1,
    };

    let user2 = User {
        email: "abc@qq.com".to_string(),
        ..user1.clone()
    };
    fn handle(user: &User) {
        println!("{}", user.active);
        println!("{}", user.email);
        println!("{}", user.sign_in_count);
        println!("{}", user.username);
    }
    handle(&user1);
    handle(&user2);

    // 元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!(
        "{black:?}, {origin:?}, {} {} {}, {} {} {}",
        black.0, black.1, black.2, origin.0, origin.1, origin.2
    );
}
