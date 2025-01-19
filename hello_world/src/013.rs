#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use core::{
    fmt::{self, Display},
    iter::{repeat, Sum},
    time::Duration,
};
use num::Complex;
use std::{
    fmt::format,
    process::Output,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
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

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // 整型溢出
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);

    // 浮点型
    let x = 2.0; // f64
    let y: f32 = 3.0; //f32
    println!("{}, {}", x, y);

    // NaN
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }

    // 数字运算
    let sum = 5 + 10;

    // 减法
    let difference = 95.4 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 求余
    let remainder = 43 % 5;

    println!(
        "{}, {}, {}, {}, {}",
        sum, difference, product, quotient, remainder
    );
    println!(
        "{:08b}, {:08b}, {:08b}, {:08b}, {:08b}, {:08b}",
        1 & 2,
        1 | 2,
        1 ^ 2,
        !2,
        8 >> 2,
        1 << 2
    );

    // 序列
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 'a'..='z' {
        println!("{}", i);
    }
    for i in 'A'..='Z' {
        println!("{}", i);
    }

    // 有理数和复数
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}", result.re, result.im);

    // 字符类型
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));

    // 布尔
    let f: bool = false;
    if f {
        println!("这是段毫无意义的代码");
    }

    // 泛型
    struct Point2<T> {
        x: T,
        y: T,
    }
    let p = Point2 { x: 1, y: 11 };

    // 枚举中使用泛型
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // 方法中使用泛型
    impl<T> Point2<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point2 { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    struct Point3<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Point3<T, U> {
        fn mixup<V, W>(self, other: Point3<V, W>) -> Point3<T, W> {
            Point3 {
                x: self.x,
                y: other.y,
            }
        }
    }

    // const 泛型
    fn display_array(arr: [i32; 3]) {
        println!("{:?}", arr);
    }
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 3] = [1, 2, 4];
    display_array(arr);

    // 常量函数
    // const fn 的基本用法
    const fn add(a: usize, b: usize) -> usize {
        a + b
    }
    const RESULT: usize = add(5, 10);
    println!("the result: is : {}", RESULT);

    // 定义特征
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }
    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }
    let post = Post {
        title: "可以的".to_string(),
        author: "非常好".to_string(),
        content: "加油!".to_string(),
    };
    let weibo = Weibo {
        username: "刘前进".to_string(),
        content: "abc".to_string(),
    };
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    pub fn notify2<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    pub fn notify3<T>(item: &T)
    where
        T: Summary + Display,
    {
        println!("Breaking news! {}", item.summarize());
    }
    fn returns_summarizable() -> impl Summary {
        Weibo {
            username: "funface".to_string(),
            content: "m1 max 太厉害了, 电脑再也不UI卡了".to_string(),
        }
    }
    trait Draw {
        fn draw(&self) -> String;
    }
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) -> String {
            "button draw".to_string()
        }
    }
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) -> String {
            "selectBox draw".to_string()
        }
    }
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }
    fn draw1(x: Box<dyn Draw>) {
        x.draw();
    }
    fn draw2(x: &dyn Draw) {
        x.draw();
    }
    let x = 1.1f64;
    let y = 8u8;
    draw1(Box::new(x));
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,
    // }
    // impl<T> Screen<T>
    // where
    //     T: Draw,
    // {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    // 调用同名的方法
    trait Pilot {
        fn fly(&self);
        fn baby_name() -> String {
            String::from("Pilot")
        }
    }
    trait Wizard {
        fn fly(&self);
        fn baby_name() -> String {
            String::from("Wizard")
        }
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furioursly*");
        }
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // 完全限定语法
    println!("A baby dog is called a {}", Human::baby_name());
    println!("A baby dog is called a {}", <Human as Pilot>::baby_name());

    // 特征定义中的特征约束
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point4 {
        x: i32,
        y: i32,
    }
    impl Display for Point4 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // 在外部类型上实现外部特征 newtype
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // 创建动态数组
    let mut v = Vec::with_capacity(10);
    v.push(1);
    let mut v = vec![1, 2, 3];
    v.push(1);

    // 浮点数数组的排序
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);

    // 生命周期
    // #[derive(Debug)]
    // struct Foo;
    // impl Foo {
    //     fn mutate_and_share(&mut self) -> &Self {
    //         &*self
    //     }
    //     fn share(&self) {}
    // }
    // let mut foo = Foo;
    // let loan = foo.mutate_and_share();
    // // foo.share();
    // println!("{:?}", foo);

    // 无界声明周期
    fn f1<'a, T>(x: *const T) -> &'a T {
        unsafe { &*x }
    }
    let a: *const i32 = &123;
    f1(a);

    // 生命周期约束
    struct DoubleRef<'a, 'b: 'a, T> {
        r: &'a T,
        s: &'b T,
    }
    fn fn_elision(x: &i32) -> &i32 {
        x
    }
    // let closure_slision = |x: &i32| -> &i32 { x };
    fn fun<T, F: Fn(&T) -> &T>(f: F) -> F {
        f
    }
    let close_siision = fun(|x: &i32| -> &i32 { x });

    // NLL
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    let r3 = &mut s;
    println!("{r3}");

    // 再借用
    #[derive(Debug)]
    struct Point5 {
        x: i32,
        y: i32,
    }
    impl Point5 {
        fn move_to(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
    }
    let mut p = Point5 { x: 0, y: 0 };
    let mut r = &mut p;
    let rr: &Point5 = &*r;
    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);

    // T:'static 和 &'static
    fn print_it<T: std::fmt::Debug + 'static>(input: &T) {
        println!("'static value passed in is: {:?}", input);
    }
    fn print_it1(input: impl std::fmt::Debug + 'static) {
        println!("'static value passwd in is: {:?}", input);
    }
    let i = 5;
    print_it(&i);
    // print_it1(&i);

    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 42;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
    }
    fn static_bound<T: Display + 'static>(t: &T) {
        println!("{t}");
    }
    println!("&'static i32: {}", r1);
    println!("&'static str: {}", r2);
    // let r3: &str;
    // {
    //     let s1 = "String".to_string();
    //     static_bound(&s1);
    //     r3 = &s1;
    // }
    // println!("{}", r3);

    // static 到底针对谁
    // {
    //     let static_string = "I'm in read-nly memory";
    //     println!("sttic_string: {}", static_string);
    // }
    // println!("static_string reference remains alive: {}", static_string);

    // 闭包
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));

    // fn muuuuu(intensity: u32) -> u32 {
    //     println!("muuuu.....");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }
    // fn workout(intensity: u32, random_number: u32) {
    //     if intensity < 25 {
    //         println!("今天活力满满, 先做 {} 个俯卧撑", muuuuu(intensity));
    //         println!(
    //             "旁边有个妹子在看, 服务窗太low, 再来 {} 组卧推",
    //             muuuuu(intensity)
    //         );
    //     } else if random_number == 3 {
    //         println!("昨天连过度了, 今天还是休息一下吧");
    //     } else {
    //         println!("昨天连过度了, 今天干干有氧, 跑步 {}", muuuuu(intensity))
    //     }
    // }
    // let intensity = 10;
    // let random_number = 2;
    // workout(intensity, random_number);

    // 闭包对内存的影响
    // 三种Fn特征
    fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool,
    {
        println!("{}", func(3));
        // println!("{}", func(4));
    }
    let x = [1, 2, 3];
    fn_once(|z| z == x.len());

    // move 关键字
    let mut s = String::new();
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}", s);

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here 's a vector: {:?}", v);
    });
    handle.join().unwrap();

    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }
    exec(update_string);
    println!("{:?}", s);

    for i in 1..100 {
       println!("{i}");
    }
}
