fn main() {
    // let mut _x = 5;
    // let (a, mut b)  = (true, false);
    // println!("a = {:?}, b = {:?}", a, b);
    // b = true;
    // assert_eq!(a, b);
    // const MAC_POINTS :u32 = 100_100;
    // println!("{}", MAC_POINTS);

    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x + 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }

    // println!("The value of x is: {}", x);

    // let space = "       ";
    // println!("{}", space);
    // let space = space.len();
    // println!("{}", space);

    // let space = "       ";
    // println!("{}", space);
    // let space = space.len();

    // println!("{}", space);

    // let guess :i32 = "42".parse().expect("Not a number");
    // println!("{}", guess);

    // let num :u8 = 255;G
    // println!("{}", num);

    // let x = 2.0;

    // let y:f32 = 3.0;

    // println!("{}, {}", x , y);

    // assert!(1 + 2 == 3);

    // let abc:(f32, f32, f32) = (0.1, 0.2, 0.3);
    // let xyz:(f64, f64, f64) = (0.1, 0.2, 0.3);
    // println!("abc (f32)");
    // println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    // println!("      0.3: {:x}",(abc.2).to_bits());
    // println!();

    // println!("xyz (f64)");
    // println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    // println!("      0.3: {:x}", (xyz.2).to_bits());
    // println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // // assert!(xyz.0 + xyz.1 == xyz.2);

    // let x = (-42.0_f32).sqrt();
    // // assert_eq!(x, x);

    // if x.is_nan() {
    //     println!("未定义的数字行为");
    // }

    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let remainder = 43 % 5;
    // println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);

    // let a :i32 = 2;
    // let b :i32 = 3;

    // println!("(a & b) value is {}", a & b);
    // println!("(a | b) value is {}", a | b);
    // println!("(a ^ b) value is {}", a ^ b);
    // println!("(!b) value is {}", !b);
    // println!("(a << b) value is {}", a << b);
    // println!("(a >> b) value is {}", a >> b);

    // let mut a = a;
    // a <<= b;
    // println!("(a << b) value is {}", a);

    // for i in 1..=5 {
    //     println!("{}", i);
    // }

    // for i in 'a'..='z' {
    //     println!("{}", i);
    // }

    // let x = '中';
    // println!("字符'中'占用{}字节的内存大小", std::mem::size_of_val(&x));

    // let t = true;
    // println!("{}", t);
    // let f :bool = false;
    // if f {
    //     println!("这是段毫无意义的代码");
    // }

    // assert_eq!(ret_uint_type(), ())

    // let x = 5u32;

    // let y = {
    //     let x_squared = x * x;
    //     let x_cube = x_squared * x;
    //     x_cube + x_squared + x
    // };

    // let z = {
    //     2 * x;
    // };

    // println!("x is {:?}", x);
    // println!("y is {:?}", y);
    // println!("z is {:?}", z);

    // let v = {
    //     let mut x :i32 = 1;
    //     x += 2;
    //     x
    // };
    // assert_eq!(v, 3);

    // let x = 3;
    // let v = x;
    // assert!(v == 3);

    // let v = (let x = 3);
    // assert!(v == 3);

    // let s = sum(1, 2);
    // assert_eq!(s, 3);

    // let mut s = "hello";
    // println!("{}", s);
    // s = "abc";
    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s2: {}, world!", s2);

    // let x = "hello world";
    // let y = x;
    // println!("{}, {}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let s = String::from("hello");
    // takes_ownership(s);
    // println!("{}", s);

    // let x = 5;
    // makes_copy(x);
    // println!("{}", x);

    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("{}, {}, {}", s3, s2, s1);

    // let x = String::from("hello, world");
    // let y = x.clone();
    // println!("{}, {}", x, y);

    // let x = "hello, world!";
    // let y = x;
    // println!("{}, {}", x, y);

    // let x = &String::from("hello, world");
    // let y = x;
    // println!("{}, {}", x, y);

    // let x = String::from("hello, world");
    // let y = x.as_str();
    // println!("{}, {}", x, y);

    // let s1 = String::from("hello, world");
    // let s2 = take_ownership(s1);
    // println!("{}", s2);

    // let s = gives_ownership();
    // println!("{}", s);

    // let s = String::from("hello, world");
    // print_str(s.clone());
    // println!("{}", s);

    // let x = (1, 2, (), "hello");
    // let y = x;
    // println!("{:?}, {:?}", x, y);

    // let s = String::from("hello, ");

    // let mut s1 = s;
    // s1.push_str("world");
    // println!("{}", s1);

    // #[derive(Debug)]
    // struct Person {
    //     name: String,
    //     age: Box<u8>,
    // }

    // let person = Person {
    //     name: String::from("Alice"),
    //     age: Box::new(20),
    // };

    // let Person { name, ref age} = person;

    // println!("The person's age is {}", age);
    // println!("The person's name is {}", name);
    // // println!("The person struct is {:?}", person);
    // println!("The person's age from person struct is {}", person.age);

    // let t = (String::from("hello"), String::from("world"));
    // let _s = t.0;
    // println!("{:?}", t.1);

    // let t = (String::from("hello"), String::from("world"));
    // let (s1, s2) = &t;
    // println!("{:?}, {:?}, {:?}", s1, s2, t);

    // let t = (String::from("hello"), String::from("world"));
    // let (s1, s2) = t.clone();
    // println!("{:?}, {:?}, {:?}", s1, s2, t);

    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}", s1, len);

    // let mut s = String::from("hello");
    // change(&mut s);

    // 同一作用域，特定数据只能有一个可变引用
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // println!("{}", r1);

    // let r2 = &mut s;
    // println!("{}", r2);

    // 很多时候，大括号可以帮我们解决一些编译不通过的问题，通过手动限制变量的作用域
    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    //     println!("{}",r1);
    // }
    // let r2 = &mut s;
    // println!("{}",r2);

    // 可变引用与不可变引用不能同时存在
    // 引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;

    // println!("{}, {}, {}", r1, r2, r3);

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{}, and {}", r1, r2);
    // let r3 = &mut s;
    // println!("{}", r3);

    // 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    // 引用必须总是有效的
    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);

    // let x = 5;
    // let p = &x;
    // println!("x 的内存地址是 {:p}", p);

    // let x = 5;
    // let y = &x;
    // assert_eq!(5, *y);

    // let mut s = String::from("hello, ");
    // borrow_object(&s)

    // let mut s = String::from("hello, ");
    // push_str(&mut s);
    // println!("{}", s);

    // let mut s = String::from("hello, ");
    // let p = &mut s;
    // p.push_str("world");

    // println!("{}", p);

    // ref 与 & 类似，可以用来获取一个值的引用，但是它们的用法有所不同。
    // let c = '中';
    // let r1 = &c;
    // let ref r2 = c;
    // assert_eq!(*r1, *r2);
    // assert_eq!(get_addr(r1), get_addr(r2));

    // let  s = String::from("hello");
    // let r1 = & s;
    // let r2 = & s;
    // println!("{}, {}", r1, r2);

    // 从不可变对象借用可变
    // let mut s = String::from("hello, ");
    // borrow_object(&mut s)

    // let mut s = String::from("hello, ");
    // borrow_object(&s);
    // s.push_str("world");

    // let mut s = String::from("hello, ");
    // let r1 = &mut s;
    // r1.push_str("world");

    // let r2 = &mut s;
    // r2.push_str("!");

    // println!("{}", r1);

    // let mut s = String::from("hello, ");
    // let r1 = &mut s;
    // // let r2 = &mut s;
    // r1.push_str("world");

    // let s = String::from("中国人");
    // let slice = &s[..3];
    // println!("{:?}", slice);

    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // // s.clear();
    // println!("the first word is: {}", word);

    // let mut s = String::from("Hello ");
    // s.push_str("rust");
    // println!("追加字符串 push_str() -> {}", s);

    // s.push('!');
    // println!("追加字符 push () -> {}", s);

    // let mut s = String::from("Hello rust!");
    // s.insert(5, ',');
    // println!("插入字符 insert() -> {}", s);
    // s.insert_str(6, "   I like");
    // println!("插入字符串 insert_str() -> {}", s);

    // let string_replace = String::from("I like rust, Learning rust is my favorite!");
    // let new_string_replace = string_replace.replacen("rust", "RUST", 1);
    // dbg!(new_string_replace);

    // let mut string_replace_range = String::from("I like rust!");
    // string_replace_range.replace_range(7..8, "R");
    // dbg!(string_replace_range);

    // let mut string_pop = String::from("rust pop 中文!");
    // let p1 = string_pop.pop();
    // let p2 = string_pop.pop();
    // dbg!(p1);
    // dbg!(p2);
    // dbg!(string_pop);

    // let mut string_remove = String::from("测试remove方法");
    // println!("string_remove 占 {} 哥字节", std::mem::size_of_val(string_remove.as_str()));
    // string_remove.remove(0);
    // string_remove.remove(2);
    // dbg!(string_remove);

    // let mut string_truncate = String::from("测试truncate");
    // string_truncate.truncate(3);
    // dbg!(string_truncate);

    // let mut string_clear = String::from("string clear");
    // string_clear.clear();
    // dbg!(string_clear);

    // let string_append = String::from("hello ");
    // let string_rust = String::from("rust");
    // let result = string_append + &string_rust;
    // let mut result = result + "!";
    // result+="!!!";
    // println!("连接字符串 + -> {}", result);

    // let s1 = String::from("hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // assert_eq!(s3, "hello, world!");
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s);

    // let s1 = "hello";
    // let s2 = String::from("rust");
    // let s = format!("{}, {}", s1, s2);
    // println!("{}", s);

    // println!("{}", "hello \\x52\\x75\\x73\\x74");
    // let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    // println!("{}", raw_str);

    // let quotes = r#"And then I said: "There is no escape!""#;
    // println!("{}", quotes);

    // let longer_delimiter = r###"A string with "#in it. And even "##!""###;
    // println!("{}", longer_delimiter);

    // for c in "中国人".chars() {
    //     println!("{}", c);
    // }

    // for b in "中国人".bytes() {
    //     println!("{}", b);
    // }

    // let s :&str = "hello, world";
    // println!("{}", s);

    // let s :Box<str> = "hello, world".into();
    // greetings(&s)

    // let mut s = String::from("");
    // s.push_str("hello, world");
    // s.push('!');
    // assert_eq!(s, "hello, world!");

    // let mut s = String::from("hello");
    // s.push(',');
    // s.push_str(" world");
    // s +="!";
    // println!("{}", s);

    // let s = String::from("I like dogs");
    // let s1 = s.replace("dogs", "cats");
    // assert_eq!(s1, "I like cats");

    // let s1 = String::from("hello,");
    // let s2 = String::from("world!");
    // let s3 = s1.clone() + &s2;
    // assert_eq!(s3, "hello,world!");
    // println!("{}", s1);

    // let s = "hello, world".to_string();
    // greetings(s);

    // let bytestring =b"this is a byte string";
    // println!("{:?}", bytestring);

    // let escaped = b"\x52\x75\x73\x74 as bytes";
    // println!("Some escaped bytes: {:?}", escaped);

    // let raw_bytestring = br"\u{211D} is not escaped here";
    // println!("{:?}", raw_bytestring);

    // if let Ok(my_str) = str::from_utf8(raw_bytestring) {
    //     println!("And the same as text:'{}'", my_str);
    // }
    // let _quotes = br#"You can also use "fancier" formatting,\ like with normal raw strings"#;
    // let shift_jis = b"\x82\xe6\x82\xb1\x82\xb1\x82\xbb";

    // match str::from_utf8(shift_jis) {
    //     OK
    // }

    // 用模式匹配解构元组
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {}", y);

    // 用 . 来访问元组
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;
    // println!("{}, {}, {}", five_hundred, six_point_four, one);

    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // let _t0:(u8,i16) = (0, -1);
    // let _t1:(u8,(i16, u32)) = (0,(-1, 1));
    // let _t :(u8,u16,i64,&str,String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    // let t = ("i", "am", "sumface");
    // assert_eq!(t.2, "sumface");

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    // println!("too long tuple: {:?}", too_long_tuple);

    // let tup = (1, 6.4, "hello");
    // let (x, z, y) = tup;
    // assert_eq!(x, 1);
    // assert_eq!(y, "hello");
    // assert_eq!(z, 6.4);

    // let (x, y) = sum_multiply((2, 3));
    // assert_eq!(x, 5);
    // assert_eq!(y, 6);

    // struct User {
    //     active:bool,
    //     username:String,
    //     email:String,
    //     sign_in_count:u64,
    // };

    // let user1 = User {
    //     email:String::from("someone@example.com"),
    //     username:String::from("someusername123"),
    //     active: true,
    //     sign_in_count:1,
    // };
    // let user2 = User {
    //     active:user1.active,
    //     username:user1.username,
    //     email:String::from("another@example.com"),
    //     sign_in_count:user1.sign_in_count,
    // };
    // println!("{}", user1.active);
    // println!("{:?}", user1);

    // let f1 = File {
    //     name: String::from("f1,txt"),
    //     data: Vec::new(),
    // };
    // let f1_name = &f1.name;
    // let f1_length = &f1.data.len();

    // println!("{:?}", f1);
    // println!("{} is {} bytes long", f1_name, f1_length);

    // 元组结构体(Tuple Struct)
    // #[derive(Debug)]
    // struct Color (i32,i32,i32);
    // #[derive(Debug)]
    // struct Point (i32,i32,i32);
    // let black = Color(0, 0, 0);
    // let origin = Point(0,0,0);
    // println!("{:?}, {:?}", black, origin);

    // 单元结构体(Unit-like Struct)
    // struct AlwaysEqual;
    // let subject = AlwaysEqual;
    // impl SomeTrait for AlwaysEqual {

    // }

    // 结构体数据的所有权
    // let user1 = User {
    //     email:"someone@eample.com",
    //     username:"someusername123",
    //     active:true,
    //     sign_in_count:1,
    // };
    // println!("{:#?}", user1);

    // #[derive(Debug)]
    // struct Rectangle {
    //     width:u32,
    //     height:u32,
    // };
    // let scale = 2;
    // let rect1 = Rectangle {
    //     width:dbg!(30 * scale),
    //     height:50,
    // };
    // println!("rect1 is {:?}", rect1);
    // dbg!(&rect1);

    // struct Person {
    //     name: String,
    //     age: u8,
    //     hobby: String,
    // }
    // let age = 30;
    // let p = Person {
    //     name: String::from("sunface"),
    //     age,
    //     hobby:"abc".to_string(),
    // };

    // let heart = PokerSuit::Hearts;
    // let diamond = PokerSuit::Diamonds;
    // print_suit(heart);
    // print_suit(diamond);

    // let c1 = PokerCard {
    //     suit: PokerSuit::Clubs,
    //     value: 1,
    // };
    // let c2 = PokerCard {
    //     suit: PokerSuit::Diamonds,
    //     value: 12,
    // };

    // let c1 = PokerCard::Spades(5);
    // let c2 = PokerCard::Diamonds(13);

    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent_number : Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
    // let a = [3;5];
    // println!("{}", a);

    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();
    // // 读取控制台的输出
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!(
    //     "The value of the element at index {} is: {}",
    //     index, element
    // );

    // let array = [String::from("rust is good!"); 8];

    // println!("{:#?}", array);

    // let array = [
    //     String::from("rust is good!"),
    //     String::from("rust is good!"),
    //     String::from("rust is good!"),
    // ];

    // println!("{:#?}", array);

    // let array:[String;8] = std::array::from_fn(|_i|String::from("rust is good!" ));
    // println!("{:#?}", array);

    // 数组切片
    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];
    // assert_eq!(slice, &[2, 3]);

    // let a = [4, 3, 2, 1];
    // // `.iter()` 方法把 `a` 数组变成一个迭代器
    // for (i, v) in a.iter().enumerate() {
    //     println!("第{}个元素是{}", i + 1, v);
    // }

    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index = index + 1;
    // }

    // let mut n = 0;

    // loop {
    //     if n > 5 {
    //         break
    //     }
    //     println!("{}", n);
    //     n+=1;
    // }

    // println!("我出来了！");

    // enum MyEnum {
    //     Foo,
    //     Bar,
    // }
    // let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    // // v.iter().filter(|x| x == MyEnum::Foo);
    // v.iter().filter(|x| matches!(x, MyEnum::Foo));

    // let foo = 'f';
    // assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    // let bar = Some(4);
    // assert!(matches!(bar, Some(x) if x > 2));

    // let x = 5;

    // match x {
    //     1..=5 => println!("one through five"),
    //     _ => println!("something else"),
    // }

    // let msg = Message::ChangeColor(0, 160, 255);

    // match msg {
    //     Message::Quit => {
    //         println!("The Quit variant has no data to destructure.")
    //     }
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {} and in the y direction {}", x, y);
    //     }
    //     Message::Write(text) => println!("Text message: {}", text),
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    //     }
    // }

    // let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    // match msg {
    //     Message::ChangeColor(Color::Rgb(r, g, b)) => {
    //         println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    //     }
    //     Message::ChangeColor(Color::Hsv(h, s, v)) => {
    //         println!(
    //             "Change the color to hue {}, saturation {}, and value {}",
    //             h, s, v
    //         )
    //     }
    //     _ => (),
    // }

    // struct Point {
    //     x: i32,
    //     y: i32,
    //     z: i32,
    // }

    // let origin = Point { x: 0, y: 0, z: 0 };

    // match origin {
    //     Point { x, .. } => println!("x is {}", x),
    // }

    // 匹配守卫提供的额外条件
    // let num = Some(4);

    // match num {
    //     Some(x) if x < 5 => println!("less than five: {}", x),
    //     Some(x) => println!("{}", x),
    //     None => (),
    // }

    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("Got 50"),
    //     Some(n) if n == y => println!("Matched, n = {}", n),
    //     _ => println!("Default case, x = {:?}", x),
    // }

    // println!("at the end: x = {:?}, y = {}", x, y);

    // enum Message {
    //     Hello { id: i32 },
    // }

    // let msg = Message::Hello { id: 5 };

    // match msg {
    //     Message::Hello {
    //         id: id_variable @ 3..=7,
    //     } => {
    //         println!("Found an id in range: {}", id_variable)
    //     }
    //     Message::Hello { id: 10..=12 } => {
    //         println!("Found an id in another range")
    //     }
    //     Message::Hello { id } => {
    //         println!("Found some other id: {}", id)
    //     }
    // }

    // #[derive(Debug)]
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // // 绑定新变量 `p`，同时对 `Point` 进行解构
    // let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    // println!("x: {}, y: {}", px, py);
    // println!("{:?}", p);

    // let point = Point { x: 10, y: 5 };
    // if let p @ Point { x: 10, y } = point {
    //     println!("x is 10 and y is {} in {:?}", y, p);
    // } else {
    //     println!("x was not 10 :(");
    // }

    // 使用模式 &mut V 去匹配一个可变引用时，你需要格外小心，因为匹配出来的 V 是一个值，而不是可变引用
    // let mut v = String::from("hello,");
    // let r = &mut v;

    // match r {
    //    mut value => value.push_str(" world!")
    // }

    // struct Circle {
    //     x:f64,
    //     y:f64,
    //     radius:f64,
    // }
    // impl Circle {
    //     fn new(x:f64, y: f64, radius: f64) -> Circle {
    //         Circle {
    //             x: x,
    //             y: y,
    //             radius:radius,
    //         }
    //     }
    //     fn area(&self) -> f64 {
    //         std::f64::consts::PI * (self.radius * self.radius)
    //     }
    // }

    // #[derive(Debug)]
    // struct Rectangle {
    //     width:u32,
    //     height:u32,
    // }
    // impl Rectangle {
    //     fn area(&self) -> u32 {
    //         self.width * self.height
    //     }
    // }
    // let rect1 = Rectangle{width: 30, height: 50};
    // println!("The area of the rectangle is {} square pixels.", rect1.area());
    // pub struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }
    // impl Rectangle {
    //     fn area(&self) -> u32 {
    //         self.width * self.height
    //     }

    //     fn can_hold(&self, other: &Rectangle) -> bool {
    //         self.width > other.width && self.height > other.height
    //     }
    // }
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // #![allow(unused)]
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {
    //         if let Message::Write(s) = self {
    //             println!("{}", s);
    //         }
    //     }
    // }

    // let m = Message::Write(String::from("hello"));
    // m.call();

    // fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    //     let mut largest = list[0];

    //     for item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // struct Point<T, U> {
    //     x:T,
    //     y:U,
    // }
    // impl <T,U> Point<T, U> {
    //     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    //         Point {
    //             x: self.x,
    //             y: other.y,
    //         }
    //     }
    // }
    // let p1 = Point {x: 5, y: 10.4};
    // let p2 = Point {x: "Hello", y: 'c'};
    // let p3 = p1.mixup(p2);
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // fn display_array<T: std::fmt::Debug, const N: usize>(arr: &[T;N]) {
    //     println!("{:?}", arr);
    // }
    // let arr: [i32; 3] = [1, 2, 3];
    // display_array(&arr);

    // let arr: [i32; 2] = [1, 2];
    // display_array(&arr);

    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }
    // pub struct Post {
    //     pub title: String,
    //     pub author:String,
    //     pub content:String,
    // }
    // impl Summary for Post {
    //     fn summarize(&self) -> String {
    //         format!("文章{}, 作者是{}", self.title, self.author)
    //     }
    // }
    // pub struct Weibo {
    //     pub username:String,
    //     pub content:String,
    // }
    // impl Summary for Weibo {
    //     fn summarize(&self) -> String {
    //         format!("{}发表了微博{}", self.username, self.content)
    //     }
    // }
    // let post = Post{title:"Rust语言简介".to_string(),author:"Sunface".to_string(), content: "Rust 棒极了".to_string()};
    // let weibo = Weibo{username:"sunface".to_string(), content:"好像微博没有Tweet好用".to_string()};
    // println!("{}", post.summarize());
    // println!("{}", weibo.summarize());

    // pub struct Post {
    //     pub title: String,
    //     pub author: String,
    //     pub content: String,
    // }
    // pub struct Weibo {
    //     pub username: String,
    //     pub content: String,
    // }
    // pub trait Summary {
    //     fn summarize(&self) -> String {
    //         String::from("(Read more...)")
    //     }
    // }
    // impl Summary for Post {}
    // impl Summary for Weibo {
    //     fn summarize(&self) -> String {
    //         format!("{}发表了微博{}", self.username, self.content)
    //     }
    // }
    // let post = Post {
    //     title: "Rust语言简介".to_string(),
    //     author: "Sunface".to_string(),
    //     content: "Rust 棒极了".to_string(),
    // };
    // let weibo = Weibo {
    //     username: "sunface".to_string(),
    //     content: "好像微博没有Tweet好用".to_string(),
    // };
    // println!("{}", post.summarize());
    // println!("{}", weibo.summarize());

    // pub fn notify(item: &impl Summary) {
    //     println!("Breaking news! {}", item.summarize());
    // }
    // notify(&post);
    // pub fn notify2<T: Summary>(_item: &T, _item2: &T) {}
    // notify2(&post, &post);

    fn largest<T:PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// use std::io;
// enum PokerCard {
//     Clubs(u8),
//     Spades(u8),
//     Diamonds(u8),
//     Hearts(u8),
// }
// #[derive(Debug)]
// enum PokerSuit {
//     Clubs,
//     Spades,
//     Diamonds,
//     Hearts,
// }
// struct PokerCard {
//     suit: PokerSuit,
//     value: u8,
// }
// fn print_suit(card: PokerSuit) {
//     println!("{:?}", card);
// }

// #[derive(Debug)]
// struct User {
//     username:&str,
//     email:&str,
//     sign_in_count:u64,
//     active:bool,
// }

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<u8>,
// }

// fn sum_multiply(nums:(i32,i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }

// fn calculate_length(s:String) -> (String,usize) {
//     let length = s.len();
//     (s, length)
// }

// fn greetings(s:String){
//     println!("{}", s);
// }

// fn greetings(s :&str) {
//     println!("{}", s);
// }

// fn first_word(s:&String) -> &str{
//     &s[..1]
// }

// fn borrow_object(s: &String){
//     println!("{}", s);
// }

// fn borrow_object(s: &mut String){
//     println!("{}", s)
// }

// fn get_addr(r:&char) -> String {
//     format!("{:p}", r)
// }

// fn push_str(s:&mut String) {
//     s.push_str("world")
// }

// fn borrow_object(s: &String){
//     *s ="abc";
//     println!("{}", s)
// }

// fn dangle() -> String {
//     let s = String::from("hello");
//     s
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn calculate_length(s:&String) ->usize {
//     s.len()
// }

// fn print_str(s :String) {
//     println!("{}", s);
// }

// fn gives_ownership() -> String {
//     let s = String::from("hello, world");
//     let _s = s.as_bytes();
//     s
// }

// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

// fn gives_ownership() -> String{
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String{
//     a_string
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn sum(x: i32, y: i32)->i32{
//     x + y
// }

// fn ret_uint_type(){
//     let x = 1;
//     let y = if x % 2 == 1 {
//         "odd"
//     } else {
//         "even"
//     };
//     let z = if x % 2 == 1 { "odd" } else { "even" };

//     println!("x: {}, y: {}, z: {}", x, y, z);
// }
