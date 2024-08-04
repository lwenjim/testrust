static LANGUAGE: &'static str = "rust";
const THRESHOLD: i32 = 10;
fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在main函数(主函数) 中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"smail"});

    // 报错 不能修改一个 const 常量
    // THRESHOLD = 5;
}
