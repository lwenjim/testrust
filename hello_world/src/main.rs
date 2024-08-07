fn main() {
    // ===========================使用 macro_rules! 来创建宏 ============
    macro_rules! say_hello {
        () => {
            // 此宏将会展开成为这个代码块里面的内容
            println!("Hello!");
        };
    }
    // 这个调用将展开成 println("Hello");
    say_hello!();

    macro_rules! create_function {
        // 此宏接受一个 ident 指示符表示的参数, 并创建一个名为: $func_name 的函数
        // ident 指示父用于变量名或者函数名
        ($func_name:ident) => {
            fn $func_name() {
                // stringify 宏把 ident 转换字符串
                println!("You called {:?}()", stringify!($func_name));
            }
        };
    }
    // 借助上述宏来创建名为 foo 和 bar 的函数
    create_function!(foo);
    create_function!(bar);
    macro_rules! print_result {
        // 此宏接受一个 expr 类型的表达式, 并将他作为字符串, 联通其他结果一起
        // 打印出来
        // expr 指示符表示表达式
        ($expression:expr) => {
            // stringify 把表达式 原样 转换成为字符串
            println!("{:?} = {:?}", stringify!($expression), $expression);
        };
    }
    foo();
    bar();
    print_result!(1u32 + 1);

    // 回想一下 代码块也是表达式
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}