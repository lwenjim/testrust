use rand::*;
use std::collections::HashMap;
#[macro_use]
extern crate microprofile;

fn some_test_func() -> i32 {
    let mut sorted_nums = vec![5, 2, 6, 4, 3, 1];
    sorted_nums.sort();
    let result = sorted_nums.iter().sum();
    result
}



fn some_func() {
    another_func();
}

fn another_func() {
    one_more_func();
}

fn one_more_func() {
    microprofile::shutdown();
}


#[allow(unused)]
fn main() {
    // Hello
    println!("Hello, world!");

    // Numbers
    let mut i = 5;
    i += 3;
    let f: f32 = 42.0;

    // Strings
    let s = "SomeString";
    let t = "SomeOtherString";
    let mut u: String = "The".to_string();
    u.push_str("ThirdString");

    // Vec; works pretty well!
    let nums = vec![1, 2, 3, 4, 5];

    // HashMap; does not work well :(
    let mut map = HashMap::<String, String>::new();
    map.insert("some_key".to_string(), "some_value".to_string());
    map.insert("some_other_key".to_string(), "some_other_value".to_string());

    // Stepping in the random crate
    let x: u8 = random();
    let y = random::<f64>();

    // Profile (C++)
    microprofile::init();
    {
        microprofile::scope!("group", "test");
        let result = some_test_func();
    }
    some_func();
    //microprofile::shutdown();

    // Goodbye
    println!("Goodbye cruel world");
}
