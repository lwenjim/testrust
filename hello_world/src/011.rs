#![allow(warnings, unused)]

fn main() {
    // 数据结构 HashMap
    use std::collections::HashMap;
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        dbg!(city);
    }

    // 数据结构 数组
    let arr = [1, 2, 3, 4]; // 类型推断 声明一个数组
    let arr = ["a", "b"]; // 所有变量可以重新赋值, 甚至不同数据类型
    let arr: [i32; 4] = [-1; 4]; // 指定默认初始值
    arr.len(); // 获取数组长度
    for i in arr {
        dbg!(i);
    } // 遍历数组

    for (i, n) in arr.iter().enumerate() {
        dbg!(i);
        dbg!(n);
        dbg!(arr[i]);
    } // 数组迭代器

    let arr: [i32; 4] = [10, 20, 30, 40]; // arr[1] = 0; // 数组不可变
    let mut arr: [i32; 4] = [10, 20, 30, 40]; // 可变数组

    // 数组作为函数参数
    fn update(mut arr: [i32; 4]) {
        for i in 0..4 {
            arr[i] = 0;
        }
        dbg!(arr);
    }
    dbg!(arr);
    update(arr); // 作为值传递
    dbg!(arr);

    let mut arr = [10, 20, 30, 40]; // 可变数组
    fn update2(arr: &mut[i32; 4]) {
        for i in 0..4 {
            arr[i] = 0;
        }
        dbg!(arr);
    }
    dbg!(arr);
    update2(&mut arr); // 作为值传递
    dbg!(arr);

    // 数据结构 元组
    let tuple = (1, "abc", false);
    dbg!(tuple.0);
    let (age, name, is_close) = (1, "abc", false);
    dbg!(age);
    dbg!(name);
    dbg!(is_close);
}

// use std::{
//     env::{self, args},
//     os,
// };

// use error_chain::error_chain;
// use select::document::Document;
// use select::predicate::Name;
// error_chain! {
//     foreign_links{
//         a(reqwest::Error);
//         b(std::io::Error);
//     }
// }
// #[tokio::main]
// async fn main() -> Result<()> {
//     let data: String = env!("PATH").parse().unwrap();
//     for str in env::args().into_iter() {
//         loadData(str.clone()).await;
//     }
//     Ok(())
// }
// async fn loadData(str: String) -> Result<()> {
//     Document::from(reqwest::get(str).await?.text().await?.as_str())
//         .find(Name("a"))
//         .filter_map(|n| {
//             let href = n.attr("href");
//             if href?.len() > 4 && href?[0..4] == *"http" {
//                 href
//             } else {
//                 None
//             }
//         })
//         .for_each(|x| println!("{}", x));
//     Ok(())
// }

// async fn world() -> String {
//     "world".to_string()
// }

// async fn hello() -> String {
//     let w = world().await;
//     format!("hello {} async from function", w)
// }

// fn main() {
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     let msg = rt.block_on(hello());
//     println!("{}", msg);
//     let _ = rt.block_on(async {
//         println!("hello world async from block");
//     });
// }

// use tokio::time::{sleep, Duration};
// #[tokio::main]
// async fn main() {
//     let jh = tokio::spawn(async {
//         sleep(Duration::from_millis(100)).await;
//         println!("hard work finished");
//     });
//     println!("mission started");
//     let _ = jh.await.unwrap();
// }

// use tokio::sync::oneshot;
// use tokio::time::{sleep, Duration};

// #[tokio::main]
// async fn main() {
//     let (tx, rx) = oneshot::channel();
//     tokio::spawn(async {
//         sleep(Duration::from_millis(100)).await;
//         println!("hard work finished");
//         tx.send("ping".to_string()).unwrap();
//     });
//     println!("mission started");
//     let _ = rx.await.unwrap();
//     println!("mission completed");
// }
