#![allow(warnings, unused)]

use core::fmt::write;
// 变量的绑定 & 常量
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
    println!("{} is {}", n, if is_big(n) { "big" } else { "smail" });

    // 报错 不能修改一个 const 常量
    // THRESHOLD = 5;

    // ====可变变量===
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("before mutation: {}", mutable_binding);

    // 正确的代码
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // 错误
    // _immutable_binding = += 1;

    // =====作用域和遮蔽======
    //此绑定生存于main函数中
    let long_lived_binding = 1;
    let long_lived_binding2 = 1;
    println!("ox{:X}", &long_lived_binding as *const i32 as usize);
    println!("ox{:X}", &long_lived_binding2 as *const i32 as usize);
    // 这是一个代码块, 比main函数拥有更小的作用域
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        //此绑定*遮掩了外面的绑定
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // 报错 short_lived_binding 在此作用域上不存在
    // println!("outer short: {}", short_lived_binding);
    //改正 注释掉这行

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样 * 遮掩个* 了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);

    // =========变量先声明=========
    // 声明一个变量绑定
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;
    // println!("another binding: {}", another_binding);

    // 改正 注释此行
    another_binding = 1;
    println!("another binding: {}", another_binding);

    // =========冻结==========
    let mut _mutable_integer = 7i32;
    {
        // 被不可变的 _mutable_integer 遮蔽
        let _mutable_integer = _mutable_integer;

        // 报错 _mutable_integer 在本作用域被冻结
        // _mutable_integer = 50;
        //
    }
    // 正常运行 _mutable_integer 在这个作用域没有冻结
    _mutable_integer = 3;
    println!("{}", _mutable_integer);

    // ===========类型转换============
    let decimal = 65.4321_f32;
    // let integer :u8 = decimal;

    // 可以显示转换
    let integer = decimal as u8;
    let character = integer as char;
    println!("casing: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转换为无符号类型T时, 会不断加上或者减去(std:T::MAX + 1)
    // 直到位于新类型T的范围内
    // 1000 已经在u16的范围内
    println!("1000  as a u16 is : {}", 1000 as u16);

    // 1000 - 256 - 256 -256 = 232
    // 事实上的处理方式是: 从最低有效位(lsb, least significant bits) 开始保留
    // 8位 , 然后剩余位置, 直到最高有效位(msb most significant bit) 都被抛弃
    // 译注: msb就是二进制的最高位, lsb 就是二进制的最低位, 按照日常习惯就是
    // 最左边以为和最右边以为.
    // println!("1000 as a u8 as: {}", 1000 as u8);

    // -1 + 256
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // 对正数, 这就是和取模一样
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时, (位操作的)结果就和 先转换到对应的无符号类型,
    // 如果msb 时1 则改值为负, 时一样的
    // 当然如果数值已经在目标类型的范围内, 就直接把他放进去
    println!("128 as a i16 is: {}", 128 as i16);
    // 128 转成u8还是128 ,但是转到 范围内, 就是直接把它放进去.
    println!("128 as a i8 is : {}", 128 as u8);

    // 重复之前的例子
    // 1000 as u8 -> 232
    // print!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是-24
    println!("232 as a i8 is : {}", 232 as u8);

    // ==========字面量=============
    // 带后缀的字面量, 其类型在初始化时已经知道了
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量, 其类型取决于如何使用他们
    let i = 1;
    let f = 1.0;

    println!("size of x in bytes: {}", std::mem::size_of_val(&x));
    println!("size of y in bytes: {}", std::mem::size_of_val(&y));
    println!("size of z in bytes: {}", std::mem::size_of_val(&z));
    println!("size of i in bytes: {}", std::mem::size_of_val(&i));
    println!("size of f in bytes: {}", std::mem::size_of_val(&f));

    // ==========类型推断===========
    // 因为有类型说明, 编译器知道 elem 的类型是 u8
    let elem = 5u8;

    // 创建一个空向量 (vector, 即不定长的, 可以增长的数组)
    let mut vec = Vec::new();
    // 现在编译器还不知道 vec 的具体类型, 只知道他是某种东西构成的向量 Vec<_>

    // 在向量中插入 elem
    vec.push(elem);
    // 啊哈 现在编译器知道vec u8的向量了 Vec<u8>
    // 试一试 注释掉 vec.push(elem) 这一行
    println!("{:?}", vec);

    // ==========别名============
    type NanoSecond = u64;
    type Inch = u64;

    type u64_t = u64;
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型别名并不能 提供额外的类型安全, 因为别名并不是新的类型
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    // from 和 into
    let my_str = "hello";
    let my_string = String::from(my_str);

    use std::convert::From;
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // ========tryFrom 和 tryInto==========
    use std::convert::TryFrom;
    use std::convert::TryInto;
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }
    // tryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // tryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // =============toString 和 fromStr==============
    use std::fmt;
    struct Circle {
        radius: i32
    }
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
    let circle = Circle{radius: 6};
    println!("{}", circle.to_string());
    
    // 一个实现 ToString 的例子
    struct Circle2 {
        radiu : i32
    }
    impl ToString for Circle2 {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radiu)
        }
    }

    let circle = Circle{radius: 6};
    println!("{}", circle.to_string());

    // 字符串 转数字
    let parsed : i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);


}
