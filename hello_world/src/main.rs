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

    calculate!(
        eval (1 + 2) * (3 / 4)
    );

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

    )
}
// ==================macro_rules -> DRY (不写重复代码)=========
use std::ops::{Add, Mul, Sub};
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
