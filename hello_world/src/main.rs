#![allow(warnings, unused)]
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

    // ============macro_rules -> 重载 =====
    // 根据你调用它的方式 test 将以不同的方式来比较 $left 和 $right
    macro_rules! test {
        ($left:expr;and $right:expr) => {
            println!(
                "{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right
            );
        };
        ($left:expr;or $right:expr) => {
            println!(
                "{:?} or {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right
            );
        };
    }
    test!(1i32 + 1 == 2i32;and 2i32 * 2 == 4i32);
    test!(true;or false);

    // ============macro_rules -> 重复 =====
    macro_rules! find_min {
        // 基本情形
        ($x:expr) => {
            $x
        };
        ($x:expr, $($y:expr),+) => (
            std::cmp::min($x, find_min!($($y), +))
        )
    }

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32 + 2u32 * 3, 4u32));

    //==================macro_rules  DSL（领域专用语言）
    macro_rules! calculate {
        (eval $e:expr) => {
            {
                let val: usize = $e;// 强制类型为整型
                println!("{} = {}", stringify!{$e}, val);
            }
        };
    }
    calculate!(
        eval 1 + 2 //看到了吧 eval 看并不是Rust 的关键字
    );

    calculate!(eval(1 + 2) * (3 / 4));

    // ===================macro_rules 可变参数接口
    macro_rules! calculate2 {
        // 单个eval 模式
        (eval $e:expr) => (
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        );

        // 递归拆解多重 eval
        (eval $e:expr, $(eval $es:expr), +) => {{
            calculate2! {eval $e}
            calculate2! {$(eval $es), +}
        }};
    }
    calculate2!(
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    );

    // ===================== 错误处理 -> panic ===================
    fn give_princess(gift: &str) {
        // 公主讨厌蛇, 所以如果公主表示厌恶的话我们要停止
        // if gift == "snake" {panic!("AAA aaaa!!!!");}
        println!("I love {}s", gift);
    }
    give_princess("teddy bear");
    give_princess("snake");

    // ===================== 错误处理 -> option 和 unwrap ===================
    // 平民(commoner)们 见多识广 收到什么礼物都应对
    // 所有礼物都是显示的实用 match 来处理
    fn give_commoter(gift: Option<&str>) {
        // 指出每种情况的做法
        match gift {
            Some("snake") => println!("Yuck I'm throwing that snake in a fire."),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No gift? Oh well."),
        }
    }

    // 养在深阖人未识得公主剪刀蛇就会 panic 恐慌
    // 这里所有的礼物都实用 unwrap 隐是地处理
    fn give_princess2(gift: Option<&str>) {
        // unwrap 在接受到 None 时将返回 panic
        // let inside = gift.unwrap();
        // if inside == "snake" {panic!("AAAAAAA!!!!!");}
        // println!("I love {}s !!!!!", inside);
    }

    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    give_princess2(food);
    give_princess2(snake);
    give_princess2(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess2(bird);
    give_princess2(nothing);

    // ===================== 错误处理 -> 使用 ? 解开 Option ===================
    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        // 如果 current_age 是 none 这将返回none
        // 如果 current_age 是 some 内部的 u8 将赋值给 next_age
        let next_age: u8 = current_age?;
        Some(format!("Next year I will be {}", next_age))
    }

    struct Person {
        job: Option<Job>,
    }

    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }
    impl Person {
        // 获取此人的工作电话号码的区号,
        fn work_phone_area_code(&self) -> Option<u8> {
            // 没有? 运算符的话 这将徐亚很多嵌套的 match 语句
            // 这将需要更多代码 尝试自己编写一下 看看那个更容易
            self.job?.phone_number?.area_code
        }
    }
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 4242423,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61));
    // ===================== 错误处理 -> 组合算子：map ===================
    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }
    #[derive(Debug)]
    struct Peeled(Food);

    #[derive(Debug)]
    struct Chopped(Food);

    #[derive(Debug)]
    struct Cooked(Food);

    //削皮 如果 没有食物, 就是返回 None 否则返回削好皮的食物
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    // 切食物 如果没有食物, 就返回none 否则返回切好的食物
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    // 亨饪食物, 这里 我们使用 map 来代替 match 以处理 各种情况
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }

    // 这个函数会完成削皮切换快亨饪一条龙 我们吧 map 串起来 以简化代码
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    // 再尝试吃食物之前确认食物是否存在时非常重要的事
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mnn. I love {:?}", food),
            None => println!("Oh no! It wasn't editable."),
        }
    }

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    // 现在让我们试试看起来更简单的 process
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_potato);
    eat(cooked_carrot);

    // ===================== 错误处理 -> 组合算子：and_then ===================
    #[derive(Debug)]
    enum Food2 {
        CordonBleu,
        Steak,
        Sushi,
    }
    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
    }

    // 我们没有制作寿司所需要的原材料 (ingredient) 有其他的原材料
    fn have_ingredients(food: Food2) -> Option<Food2> {
        match food {
            Food2::Sushi => None,
            _ => Some(food),
        }
    }

    // 我们拥有全部的食物的食谱, 除了法国蓝带猪排 (Cordon Bleu) 的
    fn have_recipe(food: Food2) -> Option<Food2> {
        match food {
            Food2::CordonBleu => None,
            _ => Some(food),
        }
    }

    // 要做一份好菜, 我们需要原材料和食谱
    // 我们可以借助一系列 match 来表达这个逻辑
    fn cookable_v1(food: Food2) -> Option<Food2> {
        match have_ingredients(food) {
            None => None,
            Some(food) => match have_recipe(food) {
                None => None,
                Some(food) => Some(food),
            },
        }
    }

    // 也可以借用 and_then 把上面的逻辑改写的更紧凑点
    fn cookable_v2(food: Food2) -> Option<Food2> {
        have_ingredients(food).and_then(have_recipe)
    }

    fn eat2(food: Food2, day: Day) {
        match cookable_v2(food) {
            Some(food) => println!("Yay! On {:?}, we get to eat {:?}.", day, food),
            None => println!("Oh no. we don't get to eat on {:?}?", day),
        }
    }

    let (cordon_bleu, steak, sushi) = (Food2::CordonBleu, Food2::Steak, Food2::Sushi);

    eat2(cordon_bleu, Day::Monday);
    eat2(steak, Day::Tuesday);
    eat2(sushi, Day::Wednesday);
}
use core::panic;
// ==================macro_rules -> DRY (不写重复代码)=========
use std::{
    fmt::format,
    ops::{Add, Mul, Sub},
};
macro_rules! assert_equal_len {
    ($a:ident, $b: ident, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr,$z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}
