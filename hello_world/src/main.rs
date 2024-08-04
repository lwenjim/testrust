#![allow(warnings, unused)]

use std::arch::x86_64::_CMP_FALSE_OQ;
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];
    // iter - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.clone().into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("len: {}", names.len());

    //iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    //===============匹配======================
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 试一试讲13天家到质数列表中
        // 匹配一个闭区间范围
        13..=19 => println!("A team"),
        // 处理其他情况
        _ => println!("Ain t special"),
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // 试一试讲其中一条分支注解
    };

    println!("{} -> {}", boolean, binary);

    // ================解构================
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);

    // match 可以解构一个元组
    match triple {
        (0, y, z) => println!("First is 0, y is {:?}, and z is {:?}", y, z),
        (1, ..) => println!("First is 1 and the rst doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    // match 可以解构枚举
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Bule!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("red: {}, green: {}, blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("hue: {}, saturation: {}, value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("hue: {}, saturation: {}, lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => {
            println!("Cyan: {}, magenta: {}, yellow: {}, key: {}", c, m, y, k)
        } // 不需要其他分支, 因为所有的情形已经都覆盖
    }
    // ===========卫语句===========
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd!"),
        _ => println!("No correlation..."),
    }
    // ===========绑定===========
    fn age() -> u32 {
        15
    }

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // 可以直接匹配 (match) 1..=12 但那样的话孩子会是几岁?
        // 相反 在1..=12分支绑定匹配值到 n 现在年龄就是可以读取了
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
    // ===========if let===========
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("This is a really long string and {:?}", i);
        }
        _ => {}
    }

    // 全部都是 option<i32> 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if let 解构读取 若 let 江number解构 some(i) 则执行
    // 语句快 {}
    if let Some(i) = number {
        println!("Matched  {:?}", i);
    }

    // 如果要致命失败情形, 就使用else
    if let Some(i) = letter {
        println!("Matched{:?}!", i);
    } else {
        // 解构失败,切换到失败的情形
        println!("Didn't match a number. let's go with a letter!");
    }

    // 提供另一种失败情况下的条件
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    }
    // 解构失败, 使用 else if 来判断是否满足上面提供的条件
    else if i_like_letters {
        println!("Didn't match a number.let's go with a letter!")
    } else {
        // 条件的值为false, 于是一下就是默认的分支
        println!("I don't like letters. let's go with an emoticon :)!");
    }

    // 使用 if let 匹配枚举
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配到了Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // 变量 b 没有匹配到 Foo::Bar 因此设么也不会打印
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // 变量 c 匹配到了 Foo::Qux 他带着一个值 就和上面例子中的 some() 类似
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // eg
    enum Foo2 {
        Bar,
    }
    let a = Foo::Bar;
    // 变量匹配 Foo::Bar
    if let Foo::Bar = a {
        // 这就是编译时发现大错误, 使用 if let 来替换他
        println!("a is foobar");
    }

    // ==============while let============
    // 将optional 设为option<i32> 类型
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit");
                    optional = None;
                } else {
                    println!("i is {:?} Try again.", i);
                    optional = Some(i + 1);
                }
            }
            // 当解构失败是退出循环
            _ => {
                break;
            } // 为什么必须写成这样子的语句呢, 肯定有更优雅的处理方式
        }
    }
    // 使用 while let 可以使这段代码变得更加优雅：
    let mut optional2 = Some(0);
    while let Some(i) = optional2 {
        if i > 9 {
            println!("Greater than 9, quit");
            optional = None;
        } else {
            println!("i is {:?} Try again.", i);
            optional = Some(i + 1);
        }
        // 使用的缩进更少, 并且不用显式地处理失败的情况.
    }
    // if let 有可选项 else / else if 分句
    // 而while let 没有
}
