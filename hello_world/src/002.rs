#![allow(warnings, unused)]

use core::fmt;

// 创建一个 enum (枚举) 来对web事件分类, 注意变量名和类型共同指定了 enum 取值的种类 PageLoad 不等于 PageUpload, KeyPress(char) 不等于Parse(String) 各个取值不同, 互相独立
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: f64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::Click { x, y } => {
            println!("clicked at x = {}, y = {}", x, y);
        }
        WebEvent::KeyPress(s) => println!("{}", s),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::Paste(s) => println!("pasted {}", s),
    }
}
#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl fmt::Display for VeryVerboseEnumOfThingsToDoWithNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned 从一个字符串切片创建一个具有所有权的string
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20f64, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    println!("{x}");
}
