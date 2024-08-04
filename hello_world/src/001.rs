#![allow(warnings, unused)]

use core::fmt;
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
// 单元结构体
struct Unit;

// 元祖结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}
impl fmt::Display for  Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.top_left, self.bottom_right)
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法常见新的point
    // 这样可以用到之前的point的字段
    let botom_right = Point { x: 15.2, y: 38f32, ..point };

    // bottom_right.y 于 point.y 一样, 因为这个字段就是从point中来的
    println!("second point: ({}, {})", botom_right.x, botom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: botom_right,
    };

    // 实例化一个单元结构
    let _unit = Unit;

    // 实例化一个元祖结构
    let pair = Pair(1, 0.01);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元祖结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let Rectangle {
        top_left: Point { x, y },
        bottom_right: Point { x: x2, y: y2 },
    } = _rectangle;

    println!("{}, {}, {}, {}", x, y, x2, y2);
    println!(
        "width: {}, height: {}, area: {}",
        x2 - x,
        y2 - y,
        (x2 - x) * (y2 - y)
    );

    let p1 = Point{x: 15f32, y: 34f32};
    let size = 12f32;

    let rec1 = Rectangle{
        top_left:p1.clone(),
        bottom_right:Point { x: p1.x+size, y: p1.y+size },
    };
    println!("{}", rec1);
}
