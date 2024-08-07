#![allow(warnings, unused)]
#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

type MathResult = Result<String, MathError>;

// 不可复制的类型
struct Empty;
struct Null;

// T 的泛型 trait
trait DoubleDrop<T> {
    // 定义一个调用者的方法, 接受一个额外的参数T,但不对他做任何事
    fn double_drop(self, _: T);
}

// 对泛型的调用类型 U 和 任何泛型类型 T 实现 DoubleDrop<T>
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

fn main() {
    let d = 1;
    let f = || -> MathResult {
        let a: Option<String> = Some("hello".to_owned());
        let b = a.clone();
        if d == 1 {
            Ok(b.expect("err"))
        } else {
            Err(MathError::DivisionByZero)
        }
    };

    let a = f().expect("err1");
    println!("{}", a);

    let empty = Empty;
    let null = Null;

    // 释放empty 和 null
    empty.double_drop(null);

    // empty;
    // null;

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array)
    compare_types(&array, &vec);

    // =======================where 分句==========================

    use std::fmt::Debug;
    trait PrintInOption {
        fn print_in_option(self);
    }

    // 这里需要一个 where 子句 否则就要表达成 T: Debug 这样意思就变了
    // 或者改用另一种间接的方法
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        // 我们要将 Option<T>: Debug 作为约束, 因为那是要打印的内容
        // 否则我们会给出错误的约束
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    vec.print_in_option();

    // ================new type 惯用法==================
    struct Years(i64);
    struct Days(i64);
    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));

    // ================关联类型==================
    struct Container(i32, i32);
    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // 显示地要求 A 和 B
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }
    impl Contains<i32, i32> for Container {
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }
    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        container.last() - container.first()
    }
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));

    // 使用关联类型优化上面的例子
    struct Container2(i32, i32);

    // 这个 trait 检查给定的 2 哥想是否存储与容器中
    // 并且能够获得容器的第一个或者最后一个值
    trait Contains2 {
        type A;
        type B;
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }
    impl Contains2 for Container2 {
        // 指出A 和 B是什么类型 如果 input 输入类型
        // 为 container(i32, i32) 那么 output (输出)类型
        // 会被确定为i32 和 i32
        type A = i32;
        type B = i32;

        // &Self::A 和 &Self::B 在这里也是合法的类型
        fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference2<C: Contains2>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;
    let container = Container2(number_1, number_2);
    println!(
        "Dose container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", number_1);
    println!("Second number: {}", number_2);

    println!("The difference is: {}", difference2(&container));

    // ================虚类型参数==================
    use std::marker::PhantomData;
    use std::ops::Add;
    // 这个虚元组结构体对 A 是泛型的, 并且带有隐藏参数 B
    #[derive(PartialEq)]
    struct PhantomTuple<A, B>(A, PhantomData<B>);

    // 这个虚类型结构体对 A 是泛型的, 并且带有隐藏西安市 B
    #[derive(PartialEq)] // 允许这种类型进行相等测试
    struct PhantomStruct<A, B> {
        first: A,
        phantom: PhantomData<B>,
    }

    // 注意: 对于类型 A 会分配存储空间, 但是 B 不会
    // 因此 B 不能参与运算
    // 这里 f32 和 f64 是隐藏参数
    // 被指定为<char, f32> dephantomTuple类型
    let _tuple: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);

    // 被指定为: <char, f64> phantomTuple 类型
    let _struct2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    //被指定为<char,f32> 的类型
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 被指定为<char, f64> 的类型
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译器错误! 类型不匹配, 所以这些值不能够比较
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);

    // 虚类型参数练习
    #[derive(Debug, Clone, Copy)]
    enum Inch {}

    #[derive(Debug, Clone, Copy)]
    enum Mm {}
    // Length 是一个带有虚类型参数Unit的类型
    // 而且对于表示长度的类型(即f64)而言Length 不是泛型的
    // f64 已经实现了 Clone 和 Copy trait
    #[derive(Debug, Clone, Copy)]
    struct Length<Unit>(f64, PhantomData<Unit>);

    // Output 必须是T<U>类型, 所以是T<U> + T<U> = T<U>.
    impl<Unit> Add for Length<Unit> {
        type Output = Length<Unit>;

        // add() 返回一个含有和新Length结构体
        fn add(self, rhs: Length<Unit>) -> Length<Unit> {
            // + 调用了针对f64 类型Add实现
            Length(self.0 + rhs.0, PhantomData)
        }
    }
    // 指定one_foot 拥有虚类型参数Inch
    let one_foot: Length<Inch> = Length(12.0, PhantomData);

    // one_meter 拥有许类型参数 Mm
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);
    // + 调用了我们对Length<Unit> 实现的add() 方法
    // 由于one_meter 而是复制他们作为 self 和 rhs
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // 加法正常执行
    println!("one_foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // 无意义的运算当然会失败
    // 编译期错误: 类型不匹配
    // let one_feter = one_foot + one_meter;
}
