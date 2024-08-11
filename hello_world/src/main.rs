#![allow(warnings, unused)]

use core::{
    fmt::write,
    panic, prelude,
    str::{self, Chars},
};
use std::{collections::HashSet, path, process::Child, string};

fn main() {
    // 箱子, 栈和堆
    {
        use std::mem;
        #[derive(Debug, Clone, Copy)]
        struct Point {
            x: f64,
            y: f64,
        }
        struct Rectangle {
            p1: Point,
            p2: Point,
        }

        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn boxed_origin() -> Box<Point> {
            // 在对上分配这个点 point, 并返回一个指向它的指针
            Box::new(Point { x: 0.0, y: 0.0 })
        }

        // 所有类型标注都不是必须的
        // 栈分配的变量
        let point: Point = origin();
        let rectangle: Rectangle = Rectangle {
            p1: origin(),
            p2: Point { x: 3.0, y: 4.0 },
        };

        // 堆分配的 rectangle
        let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
            p1: origin(),
            p2: origin(),
        });

        // 函数的输出可以装箱
        let boxed_point: Box<Point> = Box::new(origin());

        // 两层装箱
        let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
        println!(
            "Point occupies {} bytes in the stack",
            mem::size_of_val(&point)
        );
        println!(
            "Rectangle occupies {} bytes in the stack",
            mem::size_of_val(&rectangle)
        );

        // box 的宽度就是指针宽度
        println!(
            "Boxed point occupies {} bytes in the stack",
            mem::size_of_val(&boxed_point)
        );
        println!(
            "BOxed rectangle occupies {} bytes in the stack",
            mem::size_of_val(&boxed_rectangle)
        );
        println!(
            "Boxed box occupies {} bytes in the stack",
            mem::size_of_val(&box_in_a_box)
        );

        // 将包含在 boxed_point 中的数据复制到 unboxed_point
        let unboxed_point: Point = *boxed_point;
        println!(
            "Unboxed point occupies {} bytes in the stack",
            mem::size_of_val(&unboxed_point)
        );
    }
    // 动态数组 vector
    {
        // 迭代器可以被收集 vector 中
        let collected_iterator: Vec<i32> = (0..10).collect();
        println!("Collected (0..10) into: {:?}", collected_iterator);

        // vec! 宏可用来初始化一个vector
        let mut xs = vec![1i32, 2, 3];
        println!("Initial vector: {:?}", xs);

        // 在 vector 的尾部插入一个新的元素
        println!("Push 4 into the vector");
        xs.push(4);
        println!("Vector: {:?}", xs);

        // 报错 不可变 vector 不可增长
        // collected_iterator.push(0);

        // len 方法获得一个vector 的当前大小
        println!("Vector size: {}", xs.len());

        // 下标使用中括号表示 从 0 开始
        println!("Second selement: {}", xs[1]);

        // pop 移除 vector 的最后一个元素并将它返回
        println!("Pop last element: {:?}", xs.pop());

        // 超出下标范围将抛出一个 panic
        // println!("Fourth element: {}", xs[3]);

        // 迭代一个 vector 很容易
        println!("Contents of xs:");
        for x in xs.iter() {
            println!("> {}", x);
        }

        // 可以接待 vector 的同时, 使用独立变量 (i) 来记录迭代次数
        for (i, x) in xs.iter().enumerate() {
            println!("In position {} we have value {}", i, x);
        }

        // 多亏了, iter_mut 可变的 vector 在迭代的同时 其中每个值都能被修改
        for x in xs.iter_mut() {
            *x *= 3;
        }
        println!("Updated vector: {:?}", xs);
    }
    // 字符串
    {
        // 所有的类型标注都不是必须的
        // 一个对制度内存中分配的字符串的引用
        let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
        println!("Pangram: {}", pangram);

        // 逆序迭代单词, 这里并没有分配新的字符串
        println!("Words in reverse");
        for word in pangram.split_whitespace().rev() {
            println!("> {}", word);
        }

        // 复制字符到一个vector 排序并移除重复值
        let mut chars: Vec<char> = pangram.chars().collect();
        chars.sort();
        chars.dedup();

        // 创建一个空的切克增长的 String
        let mut string = String::new();
        for c in chars {
            // 在字符串的末尾插入一个字符
            string.push(c);

            // 在字符串末尾插入一个字符串
            string.push_str(", ");
        }

        // 这个缩短的字符串是原字符串的一个切片, 所以没有执行新的分配操作
        let chars_to_trim: &[char] = &[' ', ','];
        let trimed_str: &str = string.trim_matches(chars_to_trim);
        println!("Used characters: {}", trimed_str);

        // 堆分配一个字符串
        let alice = String::from("I like dogs");
        //分配新内存并存储修改过的字符串
        let bob: String = alice.replace("dog", "cat");
        println!("Alice says: {}", alice);
        println!("Bob says: {}", bob);
    }
    // 字面量与转义字符
    {
        // 通过转义, 可以用十六进制值来表示字符
        let byte_escape = "I'm writing \x52\x75\x73\x74";
        println!("What are you doing\x3f (\\x3f means ?) {}", byte_escape);

        // 也可以使用unicode 码位表示
        let unicode_code_point = "\u{211d}";
        let character_name = "double-struck capital r\"";

        println!(
            "Unicode character {} (U+211d) is called {}",
            unicode_code_point, character_name
        );

        let long_string = "String literals can span multiple lines. The linebreak and indentation hre -> <- can be escaped tool!";
        println!("{}", long_string);

        // 原样输出
        let raw_str = r"Escapes don't work here: \x3f \u{211d}";
        println!("{}", raw_str);

        // 如果你要在原始字符串中写引号, 请在两边加一对 #
        let quotes = r#"And the I said: "There is no escape!""#;
        println!("{}", quotes);

        // 如果字符串中需要些 #,那就在定界符中使用更多的 #
        // 可使用 # 的数目没有限制
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);

        // 注意这并不是一个 &str
        let bytestring: &[u8; 20] = b"this is a bytestring";

        // 字节串没有实现 display 所以它们的打印功能有些受限
        println!("A bytestring: {:?}", bytestring);

        // 字符串可以使用单字节的转义字符...
        let escaped = b"\x52\x75\x73\x74";
        // 但不能使用 unicode 转义字符
        // let escaped = b"\u{211d} is not allowd";
        println!("Some escaped bytes: {:?}", escaped);

        // 原始字节串和原始字符串的写法一样
        let raw_bytestring = br"\u{211d} is not escaped here";
        println!("{:?}", raw_bytestring);

        // 把字节串转换为 &str 可鞥失败
        if let Ok(my_str) = str::from_utf8(raw_bytestring) {
            println!("And the same as text: '{}'", my_str);
        }

        let quotes = br#"You can also use "fancier" formatting, \
        like with normal raw strings"#;
        // 字节串不可以使用utf-8 编码
        let shift_jis = b"\x82\xe6\x82\xa8\xb1\x82";

        // 但这样的话他们就无法转成 &str了
        match str::from_utf8(shift_jis) {
            Ok(my_str) => println!("Conversion successful: '{}'", my_str),
            Err(e) => println!("Conversion failed: {:?}", e),
        }

        // 选项
        {
            // 不会panic 的证书除法
            fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
                if divisor == 0 {
                    // 失败表示成 none
                    None
                } else {
                    Some(divisor / divisor)
                }
            }
            // 此函数处理可能失败的除法
            fn try_division(dividend: i32, divisor: i32) {
                // option 值可以进行模式匹配, 就和其他枚举类型一样
                match checked_division(dividend, divisor) {
                    None => println!("{} / {} failed ", dividend, divisor),
                    Some(quotient) => {
                        println!("{} / {} = {}", dividend, divisor, quotient)
                    }
                }
            }
            try_division(4, 2);
            try_division(1, 0);

            // // 绑定 none 到一个变量需要类型标注
            // let none: Option<i32> = None;
            // let _equivalent_none = None::<i32>;

            // let optional_float = Some(0f32);

            // // 解包some 将取出被包装的值
            // println!(
            //     "{:?} unwraps to {:?}",
            //     optional_float,
            //     optional_float.unwrap()
            // );

            // 解包 none 将引发 panic
            // println!("{:?} unwraps to {:?}", none, none.unwrap())
        }
        // Result
        {
            mod checked {
                // 我们想要捕获的数学 错误
                #[derive(Debug)]
                pub enum MathError {
                    DivisionByZero,
                    NegativeLogarithm,
                    NegativeSquareRoot,
                }
                pub type MathResult = Result<f64, MathError>;
                pub fn div(x: f64, y: f64) -> MathResult {
                    if y == 0.0 {
                        // 此操作将会失败, name (与其让程序崩溃) 不如吧失败的原因包装在
                        // err 中并返回
                        Err(MathError::DivisionByZero)
                    } else {
                        Ok(x / y)
                    }
                }
                pub fn sqrt(x: f64) -> MathResult {
                    if x < 0.0 {
                        Err(MathError::NegativeSquareRoot)
                    } else {
                        Ok(x.sqrt())
                    }
                }

                pub fn ln(x: f64) -> MathResult {
                    if x < 0.0 {
                        Err(MathError::NegativeLogarithm)
                    } else {
                        Ok(x.ln())
                    }
                }
            }

            // op(x, y) === sqrt(ln(x / y))
            fn op(x: f64, y: f64) -> f64 {
                // 这是一个三层的match 金字塔
                match checked::div(x, y) {
                    Err(why) => panic!("{:?}", why),
                    Ok(ratio) => match checked::ln(ratio) {
                        Err(why) => panic!("{:?}", why),
                        Ok(ln) => match checked::sqrt(ln) {
                            Err(why) => panic!("{:?}", why),
                            Ok(sqrt) => sqrt,
                        },
                    },
                }
            }
            // println!("\n\n{}", op(1.0, 10.0));
        }
        // 散列表 HashMap
        {
            use std::collections::HashMap;
            fn call(number: &str) -> &str {
                match number {
                    "798-1364" => {
                        "We're sorry, the call cannot be completed as dialed.
                    Please hang up hand try again."
                    }
                    "645-7689" => {
                        "Hello, this is Mr. Awesome's Pizza, My name is Fred.
                    What can I get for you today?"
                    }
                    _ => "Hi! Who is this again?",
                }
            }

            let mut contacts = HashMap::new();

            contacts.insert("Daniel", "798-1364");
            contacts.insert("Ashley", "645-7689");
            contacts.insert("Katie", "435-8291");
            contacts.insert("Robert", "95601745");

            // 接受一个引用并返回Option<&v>
            match contacts.get(&"Daniel") {
                Some(&number) => println!("\n\nCalling Daniel: {}", call(number)),
                _ => println!("Don't have Daniel's number."),
            }

            // 如果被插入的值为新的内容, name hashmap::insert() 返回none
            // 否则返回 some(value)
            contacts.insert("Daniel", "164-6743");

            match contacts.get(&"Ashley") {
                Some(&number) => println!("calling ashley: {}", call(number)),
                _ => println!("Dot'nt have ashley's number"),
            }

            contacts.remove("Ashley");

            // hashmap::iter() 返回一个迭代器 该迭代器以任意顺序举出
            // (&'a key, &'a value) 对
            for (contact, &number) in contacts.iter() {
                println!("calling {}: {}", contact, call(number));
            }
        }
        // 更改或自定义关键字类型
        {
            use std::collections::HashMap;

            // eq 要求你对此类型推到 partiaeq
            #[derive(PartialEq, Eq, Hash)]
            struct Account<'a> {
                username: &'a str,
                password: &'a str,
            }
            struct AccountInfo<'a> {
                name: &'a str,
                email: &'a str,
            }
            type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;
            fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
                println!("Username: {}", username);
                println!("Password: {}", password);
                println!("Attemption logon...");

                let login = Account { username, password };

                match accounts.get(&login) {
                    Some(account_info) => {
                        println!("successful logon !");
                        println!("name: {}", account_info.name);
                        println!("email: {}", account_info.email);
                    }
                    _ => println!("login failed"),
                }
            }

            let mut accounts: Accounts = HashMap::new();
            let account = Account {
                username: "j.everyman",
                password: "password123",
            };
            let account_info = AccountInfo {
                name: "John Everyman",
                email: "j.everymn@email.com",
            };
            accounts.insert(account, account_info);
            println!("");
            try_logon(&accounts, "j.everyman", "password123");
            try_logon(&accounts, "j.everyman", "password123");
        }
        // 散列集 HashSet
        {
            use std::collections::HashMap;
            let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
            let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

            assert!(a.insert(4));
            assert!(a.contains(&4));

            // 如果值已经存在, 那么 hashset::insert() 返回 false
            // assert!(b.insert(4), "Value 4 is already in set B!");

            b.insert(5);

            // 若一个集合的元素类型实现了 debug name该集合页就实现了debug
            // 这通常将元素打印成这样子的格式 [elem1, elem2]
            println!("A: {:?}", a);
            println!("B: {:?}", b);

            // 乱序打印 [1, 2, 3, 4, 5]
            println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

            // 这将会打印出 [1]
            println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

            // 打印[1, 5]
            println!(
                "Symmetric Difference: {:?}",
                a.symmetric_difference(&b).collect::<Vec<&i32>>()
            );
        }
        // 引用计数 Rc
        {
            use std::rc::Rc;
            let rc_examples = "Rc examples".to_string();
            {
                println!("---- rc_a is created ----");
                let rc_a: Rc<String> = Rc::new(rc_examples);
                println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));
                {
                    println!("--- rc_a is cloned to rc_b ---");
                    let rc_b: Rc<String> = Rc::clone(&rc_a);
                    println!("reference count of rc_b: {}", Rc::strong_count(&rc_b));
                    println!("reference count of rc_a: {}", Rc::strong_count(&rc_a));

                    // 如果两者内部的值相等的话, 则两个rc 相等
                    println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

                    // 我们可以直接使用知道方法
                    println!("length of the value inside rc_a: {}", rc_a.len());
                    println!("value of rc_b: {}", rc_b);

                    println!("--- rc_b is dropped out of scope ---");
                }

                println!("reference cuont of rc_a: {}", Rc::strong_count(&rc_a));
                println!("---- rc_a is ropped out of scope ---");
            }
        }
        // 共享引用计数 Arc
        {
            use std::sync::Arc;
            use std::thread;
            // 这个变量声明来用指定其他值的地方
            let apple = Arc::new("the same apple");
            for i in 0..10 {
                // 这里没有数值说明, 因为他是一个指向内存对中引用的指针
                let apple = Arc::clone(&apple);

                thread::spawn(move || {
                    // 由于使用了 arc 线程 可以使用分配在arc变量指针位置的来生成
                    println!("inex:{}, {:?}", i, apple);
                });
            }
        }
        // ======================标准库更多介绍==========================
        {
            use std::thread;
            static NTHREADS: i32 = 10;

            // 这是主 (main) 线程
            // 提供一个vector 来存放所创建的子线程
            let mut children = vec![];
            for i in 0..NTHREADS {
                // 启动 另一个线程
                children.push(thread::spawn(move || {
                    println!("this is thread number {}", i)
                }))
            }
            println!("1");
            for child in children {
                // 等待线程结束, 返回一个结果
                println!("{:?}", child);
                let _ = child.join();
            }
            // 测试实例
            {
                // 这是我们要处理的数据。
                // 我们会通过线程实现 map-reduce 算法，从而计算每一位的和
                // 每个用空白符隔开的块都会分配给单独的线程来处理
                //
                // 试一试：插入空格，看看输出会怎样变化！
                let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
                // 创建一个向量, 用于储存将要创建的子线程
                let mut children = vec![];
                // 把数据分段, 每段将会单独计算
                // 每段都是完整的数据的一个引用
                let chunked_data = data.split_whitespace();

                // 对分段的数据进行迭代
                // .enumerate() 会把当前的迭代计算与被迭代的元素以元组(index, element)
                // 的形式返回, 接着立即使用 解构赋值, 将该元组解构成两个变量
                // i 和 data_segment
                for (i, data_segment) in chunked_data.enumerate() {
                    println!("data segment {} is \"{}\"", i, data_segment);

                    // 用单独的线程处理每一段数据
                    // spawn 返回新线程的句柄handle , 我们必须拥有句柄
                    // 才能活动区线程的返回值

                    // move || -> ui32 语法表示该关闭
                    // * 没有参数 ||
                    // * 会获取所有捕获变量的所有权
                    // * 返回无符号 32 位整数 -> u32
                    // rust 可以根据闭包的内容推断出 -> u32, 所以我们可以不写他
                    children.push(thread::spawn(move || -> u32 {
                        // 计算该段的每一位的和
                        let result = data_segment
                            // 对该段中的字符进行迭代
                            .chars()
                            // 把字符转成数字
                            .map(|c| c.to_digit(10).expect("should be a digit"))
                            // 对返回的数字类型的迭代求和
                            .sum();

                        // println 会锁住标准输出, 这样各线程打印店内容不会交错在一起
                        println!("processed segment {}, result = {}", i, result);

                        // 不需要return因为 rust 是一种表达式语言 每个代码块中
                        // 最后求职的表达式是代码块的值
                        result
                    }));
                }
                // 把每个线程产生的中间结果收入一个新的向量中
                let mut intermediate_sums = vec![];
                for child in children {
                    // 收集每个子线程的返回值
                    let intermediate_sum = child.join().unwrap();
                    intermediate_sums.push(intermediate_sum);
                }

                // 把所有中间结果连起来, 写法 ::<> 来位 sum() 提供类型提示
                let final_result = intermediate_sums.iter().sum::<u32>();
                println!("final sum result: {}", final_result);
            }
        }
        // 通道
        {
            use std::sync::mpsc;
            use std::sync::mpsc::{Receiver, Sender};
            use std::thread;

            static NTHREADS: i32 = 3;

            // 通道有两个端点 sender<T> receiver<T> 其中 T 是要发送
            // 的消息的类型 (类型标注时可选的)
            let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

            for id in 0..NTHREADS {
                // sender 端可被复制
                let thread_tx = tx.clone();

                // 每个线程都将通过通道来发送他的id
                thread::spawn(move || {
                    // 被创建的线程取得 thread_tx 的所有权
                    // 每个线程都把消息放在通道的消息队列中
                    thread_tx.send(id).unwrap();

                    // 发送时一个非阻塞 non-blocking 操作, 线程将发送完消息后
                    // 会立即继续进行
                    println!("thread {} finished", id);
                });
            }

            // 所有消息都在此处被收集
            let mut ids = Vec::with_capacity(NTHREADS as usize);
            for _ in 0..NTHREADS {
                // recv 方法从通道中拿到一个消息
                // 若无可用消息的话, recv 将阻止当前线程
                ids.push(rx.recv());
            }
            // 显示消息被发送的次序
            println!("{:?}", ids);
        }
        // 路径
        {
            use std::path::Path;
            // 从&'static str 创建一个 path
            let path = Path::new(".");

            // display 方法返沪一个可显示 (showable) 的结构体
            let display = path.display();

            // join 使用操作系统特定的分隔符来合并路径到一个字节让其, 并返回新的路径
            let new_path = path.join("a").join("b");

            // 将路径转换成一个字符串切片
            match new_path.to_str() {
                None => panic!("new path is not a valid utf-8 sequence"),
                Some(s) => println!("new path is {}", s),
            }
        }
        // 打开文件 open
        {
            use std::fs::File;
            use std::io::prelude::*;
            use std::path::Path;

            // 创建指向所需的文件的路径
            let path = Path::new("/tmp/hello.txt");
            let display = path.display();

            // 以只读方式打开路径, 返回 io::result<file>
            let mut file = match File::open(&path) {
                // io::error 的 description 方法返回一个描述错误的字符串
                Err(why) => panic!("could't open {}: {:?}", display, why),
                Ok(file) => file,
            };

            // 读取文件内容到一个字符串, 返回 io::result<usize>
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("could't read {}: {:?}", display, why),
                Ok(_) => print!("{}, contains:\n{}", display, s),
            }
            // file 离开作用域 并且hello.txt 文件将被关闭
        }
        // 创建文件 create
        {
            static LOREM_IPSUM: &'static str =
                "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";
            use std::fs::File;
            use std::io::prelude::*;
            use std::path::Path;

            let path = Path::new("/tmp/hello.txt");
            let display = path.display();

            // 以只读模式打开文件, 返回 io::result<file>
            let mut file = match File::create(&path) {
                Err(why) => panic!("could't create {}: {:?}", display, why),
                Ok(file) => file,
            };

            // 将 LOREM_IPSUM 字符串写进file, 返回 io::result<()>
            match file.write_all(LOREM_IPSUM.as_bytes()) {
                Err(why) => {
                    panic!("could't write to {}: {:?}", display, why);
                }
                Ok(_) => println!("successfully wrote to {}", display),
            }
        }

        // 读取行
        {
            use std::fs::File;
            use std::io::{self, BufRead};
            use std::path::Path;

            fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
            where
                P: AsRef<Path>,
            {
                let file = File::open(filename)?;
                Ok(io::BufReader::new(file).lines())
            }

            // 在生产输出之前, 文件主机必须村子与当前路径中
            if let Ok(lines) = read_lines("/tmp/hello.txt") {
                // 使用迭代器, 返回一个(可选)字符串
                for line in lines {
                    if let Ok(ip) = line {
                        println!("{}", ip);
                    }
                }
            }
        }
        // 子进程
        {
            use std::process::Command;
            let output = Command::new("rustc")
                .arg("--version")
                .output()
                .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                print!("rustc successed and stdout was:\n{}", s);
            } else {
                let s = String::from_utf8_lossy(&output.stderr);
                print!("rustc failed and stderr was: \n{}", s);
            }
        }
        // 管道
        {
            use std::io::prelude::*;
            use std::process::{Command, Stdio};

            static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

            // 启动 wc 命令
            let process = match Command::new("wc")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
            {
                Err(why) => panic!("could't spawn wc: {:?}", why),
                Ok(process) => process,
            };

            // 将字符串写入wc 的stdin
            // stdin 拥有 option<childstdin> 类型 不过我们已经知道这个实力不为空值
            // 因而可以直接 unwrap 它
            match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
                Err(why) => panic!("could't write to wc stdin: {:?}", why),
                Ok(_) => println!("sent pangram to wc"),
            }

            // 因为stdin 在上面调用就不在存活 所以它被drop 了, 管道也就关闭
            // 这点非常重要, 因为否则 wc 就不会开始处理我们刚刚发送的输入
            // stdout 字段也拥有 option<childstdout> 类型, 所以必须解包
            let mut s = String::new();
            match process.stdout.unwrap().read_to_string(&mut s) {
                Err(why) => panic!("couldn't read wc stdout: {:?}", why),
                Ok(_) => print!("wc responsed with:\n{}", s),
            }
        }
        // 等待
        {
            // use std::process::Command;
            // let mut child = Command::new("sleep").arg("5").spawn().unwrap();
            // let _result = child.wait().unwrap();
            // println!("reached end of main");
        }
        // 文件系统操作
        {
            use std::fs;
            use std::fs::{File, OpenOptions};
            use std::io;
            use std::io::prelude::*;
            use std::os::unix;
            use std::path::Path;

            // % cat path 的简单实现
            fn cat(path: &Path) -> io::Result<String> {
                let mut f = File::open(path)?;
                let mut s = String::new();
                match f.read_to_string(&mut s) {
                    Ok(_) => Ok(s),
                    Err(e) => Err(e),
                }
            }

            // % echo s > path 的简单实现
            fn echo(s: &str, path: &Path) -> io::Result<()> {
                let mut f = File::create(path)?;
                f.write_all(s.as_bytes())
            }

            // % touch path 的简单实现(忽略已存在的文件)
            fn touch(path: &Path) -> io::Result<()> {
                match OpenOptions::new().create(true).write(true).open(path) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }

            println!("mkdir a");
            // 创建一个目录 返回io::result<()>
            match fs::create_dir("a") {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(_) => {}
            }

            println!("echo hello > a/b.txt");
            //前面的匹配可以用 unwrap_or_else方法简化
            echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });

            println!("mkdir -p a/c/d");
            //递归地创建一个目录, 返回 io::result<()>
            fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });

            println!("touch a/c/e.txt");
            touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
            println!("ln -s ../b.txt a/c/b.txt");
            // 创建一个符号链接,返回io::result<()>
            if cfg!(target_family = "unix") {
                unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });
            }
            println!("cat a/c/b.txt");
            match cat(&Path::new("a/c/b.txt")) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(s) => println!("> {}", s),
            }
            println!("ls a");
            // 读取目录的内容 返回io::result<Vec<Path>>
            match fs::read_dir("a") {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(paths) => {
                    for path in paths {
                        println!("{:?}", path.unwrap().path());
                    }
                }
            }
            println!("rm a/c/e.txt");
            //删除一个文件, 返回io::result<()>
            fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
                println!("! {:?}", why);
            });

            println!("rmdir a/c/d");
            fs::remove_dir("a/c/d").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });

            use std::env;
            let args: Vec<String> = env::args().collect();
            // 第一个参数是嗲用本程序的路径
            println!("my path is {}", args[0]);

            // 其余参数是被传递给程序的命令行参数
            // 请这样调用程序
            // $ ./args arg1 arg2
            println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
        }
        // 参数解析
        {
            use std::env;
            fn increase(number: i32) {
                println!("{}", number + 1);
            }
            fn decrease(number: i32) {
                println!("{}", number - 1);
            }
            fn help() {
                println!(
                    "usage:
                match_args <string>
                    check whether given string is the answer.
                    match_args {{increase|decrease}} <integer>
                    increase or decrease given integer by one.
                "
                )
            }

            let args: Vec<String> = env::args().collect();
            match args.len() {
                // 没有传入参数
                1 => {
                    println!("my name is match_args. try passing some arguments!");
                }
                2 => match args[1].parse() {
                    Ok(42) => println!("this is the answer!"),
                    _ => println!("this is not the answer."),
                },
                // 传入一条命令和一个参数
                3 => {
                    let cmd = &args[1];
                    let num = &args[2];
                    //解析数字
                    let number: i32 = match num.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("error: second argument not an integer");
                            help();
                            return;
                        }
                    };
                    // 命令解析
                    match &cmd[..] {
                        "increase" => increase(number),
                        "decrease" => decrease(number),
                        _ => {
                            println!("error: invalid command");
                            help();
                        }
                    }
                }
                // 所有其他情况
                _ => {
                    // 显示帮助信息
                    help();
                }
            }
        }
        // 外部语言函数接口
        {
            use std::fmt;
            // 这个extern 代码块链接到libm 库
            #[link(name = "m")]
            extern "C" {
                // 这个外部函数用于计算单精度负数的平方根
                fn csqrtf(z: Complex) -> Complex;

                // 这个用来计算单精度负数的复变余弦
                fn ccosf(z: Complex) -> Complex;
            }
            // 由于调用其他语言的函数被认为是不安全的, 我们通常会给他们写一层安全的封装
            fn cos(z: Complex) -> Complex {
                unsafe { ccosf(z) }
            }

            // z = -1 + oi
            let z = Complex { re: -1., im: 0. };

            // 调用外部语言函数是布安全的操作
            let z_sqrt = unsafe { csqrtf(z) };

            println!("the square root of {:?} is {:?}", z, z_sqrt);

            // 调用不安全操作的安全的 API 封装
            println!("cos({:?}) = {:?}", z, cos(z));

            //单精度负数的最简实现
            #[repr(C)]
            #[derive(Clone, Copy)]
            struct Complex {
                re: f32,
                im: f32,
            }
            impl fmt::Debug for Complex {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    if self.im < 0. {
                        write!(f, "{}-{}i", self.re, -self.im)
                    } else {
                        write!(f, "{} + {}i", self.re, self.im)
                    }
                }
            }
        }
        // 单元测试
        mod aaa {
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            // 这个加法函数写得很差, 本例中我们会使他失败
            fn bad_add(a: i32, b: i32) -> i32 {
                a - b
            }
            #[cfg(test)]
            mod tests {
                // 注意这个惯用法: 在 tests 模块中, 从外部作用域导入所有名字
                use super::*;
                #[test]
                fn test_add() {
                    assert_eq!(add(1, 2), 3);
                }
                #[test]
                fn test_bad_add() {
                    // 这个断言会导致测试失败, 注意私有的函数也可以被测试
                    assert_eq!(bad_add(1, 2), 3);
                }
            }
        }
        // 测试 panic
        mod ur {
            pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
                if b == 0 {
                    panic!("Divide-by-zero error");
                } else if a < b {
                    panic!("Divide result is zero");
                }
                a / b
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn test_devide() {
                    assert_eq!(divide_non_zero_result(10, 2), 5);
                }
                #[test]
                #[should_panic]
                fn test_any_panic() {
                    divide_non_zero_result(1, 0);
                }

                #[test]
                #[should_panic(expected = "Divide result is zero")]
                fn test_specific_panic() {
                    divide_non_zero_result(1, 10);
                }
            }
        }
        // 开发依赖
        mod yyy {
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            #[cfg(test)]
            mod tests {
                use super::*;
                use pretty_assertions::assert_eq; // 仅仅用于测试, 不能在飞测试代码中使用

                #[test]
                fn test_add() {
                    assert_eq!(add(2, 3), 5);
                }
            }
        }
        // 不安全操作
        {
            let raw_p: *const u32 = &10;
            unsafe {
                assert!(*raw_p == 10);
            }

            use std::slice;
            let some_vector = vec![1, 2, 3, 4];
            let pointer = some_vector.as_ptr();
            let length = some_vector.len();
            unsafe {
                let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
                assert_eq!(some_vector.as_slice(), my_slice);
            }
        }
        // 原始标志符
        {
            // extern crate foo;
            // foo::try();
            // foo::r#try();
        }
    }

    {
        let john = Person::new("John");
        john.hello();
    }
}
/// 第一行是对函数的简短描述。
///
/// 接下来数行是详细文档。代码块用三个反引号开启，Rust 会隐式地在其中添加
/// `fn main()` 和 `extern crate <cratename>`。比如测试 `doccomments` crate：
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 文档注释通常可能带有 "Examples"、"Panics" 和 "Failures" 这些部分。
///
/// 下面的函数将两数相除。
///
/// # Examples
///
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// 如果第二个参数是 0，函数将会 panic。
///
/// ```rust,should_panic
/// // panics on division by zero
/// doccomments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_aa() {
        assert_eq!(1, 1);
    }
    #[test]
    fn test_bb() {
        assert_eq!(1, 1);
    }
}

// #![crate_name = "doc"]

/// 这里给出一个“人”的表示
pub struct Person {
    /// 一个人必须有名字（不管 Juliet 多讨厌她自己的名字）。
    name: String,
}

impl Person {
    /// 返回具有指定名字的一个人
    ///
    /// # 参数
    ///
    /// * `name` - 字符串切片，代表人的名字
    ///
    /// # 示例
    ///
    /// ```
    /// // 在文档注释中，你可以书写代码块
    /// // 如果向 `rustdoc` 传递 --test 参数，它还会帮你测试注释文档中的代码！
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// 给一个友好的问候！
    /// 对被叫到的 `Person` 说 "Hello, [name]" 。
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}
