fn main() {
    #![allow(warnings, unused)]
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        // 静态方法签名 Self 表示实现着类型 implement type
        fn new(name: &'static str) -> Self;

        // 实例方法签名; 这些方法将返回一个字符串
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // trait 可以提供默认的方法定义
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // 实现者可以使用它的trait方法
                println!("{} is already naked ...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);
                self.naked = true;
            }
        }
    }

    impl Animal for Sheep {
        fn new(name: &'static str) -> Self {
            Sheep {
                naked: false,
                name: name,
            }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "aaaaaa"
            } else {
                "bbbbb"
            }
        }

        fn talk(&self) {
            // 例如我们可以增加一些安静的沉思
            println!("{} pause briefly ... {}", self.name, self.noise());
        }
    }
    // 这种情况需要类型标注
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    // ======================= 特征 trait -> 使用 dyn 返回 trait ===============
    struct Sheep2 {}
    struct Cow {}

    trait Animal2 {
        // 实例方法签名
        fn noise(&self) -> &'static str {
            "abc"
        }
    }

    impl Animal2 for Sheep2 {
        fn noise(&self) -> &'static str {
            "bbb"
        }
    }
    impl Animal2 for Cow {
        fn noise(&self) -> &'static str {
            "ccc"
        }
    }
    // 返回一些实现 animal的结构体 , 但是在编译期我们不知道哪个结构体
    fn random_animal(random_number: f64) -> Box<dyn Animal2> {
        if random_number < 0.5 {
            Box::new(Sheep2 {})
        } else {
            Box::new(Cow {})
        }
    }
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You 've randomly chosen an animal, and it says {}",
        animal.noise()
    );

    // ======================= 特征 trait -> 运算符重载 ===============
    use std::ops;

    struct Foo;
    struct Bar;
    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    // std::ops::Add trait 用来指明 + 的功能, 这就是我们实现 Add<Bar> 它是用于
    // 把对象和 Bar类型的右操作数(RHS) 加起来的 trait
    // 下面的代码块实现了, Foo + Bar = FooBar 这样的运算
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;
        fn add(self, rhs: Bar) -> Self::Output {
            println!("> Foo.add(Bar) was called");
            FooBar
        }
    }

    // 通过颠倒类型, 我们实现了不服从交换率的加法
    // 这里我们实现 Add<Foo> 它是用于把对象和Foo类型右操作加起来的trait
    // 下面的代码块实现了 Bar + Foo = BarFoo 这样的运算
    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;
        fn add(self, rhs: Foo) -> Self::Output {
            println!("> Bar.add(Foo) was called");
            BarFoo
        }
    }
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);

    // ============ 特征 trait -> Drop ==============
    struct Droppable {
        name: &'static str,
    }

    // 这个简单的Drop 实现添加了打印到控制台的功能
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> dropping {}", self.name);
        }
    }
    let _a = Droppable { name: "a" };
    // 代码块 A
    {
        let _b = Droppable { name: "b" };
        // 代码块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A")
    }
    println!("Just exited block A");

    // 变量可以手动使用drop 函数来销毁
    drop(_a);

    println!("end of the main function");

    // _a 不会再这里再次销毁 ,因为它已经被(手动)销毁

    // ============ 特征 trait -> iterator ==============
    struct Fibonacci {
        curr: u32,
        next: u32,
    }
    // 为 Fibonacci (斐波那锲) 实现Iterator
    // iterator trait 只需定义一个能返回 next (下一个) 元素的方法
    impl Iterator for Fibonacci {
        type Item = u32;

        // 我们在这里使用 .curr 和 .next 来定义数列
        // 返回类型为 Option<T>
        // 当Iterator 结束时 返回NONE
        // 其他情况 返回被some包裹的下一个值
        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            // 既然斐波那契数列不存在终点 那么iterator 将不可能
            // 返回None 而总是返回Some
            Some(self.curr)
        }
    }

    // 返回一个斐波那契数列生成器
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }

    // 0..3 是一个 Iterator 会产生 0, 1, 和 2
    let mut sequence = 0..3;
    println!("Four consecutive next calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // for 遍历 Iterator 直到返回None
    // 并且每个 some 值都被解包 然后绑定一个变量,
    println!("Iterate through 0..3 using for");
    for i in 0..3 {
        println!("> {}", i);
    }

    // take(n) 方法提取 Iterator 的前 n 项
    println!("The first four terms of the Fibonacci sequence are:");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // skip(n) 方法 移除前 n 项 从而缩短了 Iterator
    println!("The next four terms of the fbonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }
    let array = [1u32, 3, 3, 7];

    // iter 方法对数组/slice 产生一个 Iterator
    for i in array.iter() {
        println!("> {}", i);
    }

    // ============ 特征 trait -> impl trait ==============
    use std::iter;
    use std::vec::IntoIter;

    // 该函数组合了两个 Vec<i32> 并在其上返回一个迭代器
    // 看看它的返回类型多么复杂
    fn combine_vecs_explicit_return_type(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    // 这是完全相同的函数 , 但其返回类型使用 impl trait
    fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    // 返回一个将输入和 y 相加的函数
    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| x + y;
        closure
    }

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
    }

    // ============ 特征 trait -> clone ==============
    #[derive(Debug, Clone, Copy)]
    struct Nil;

    // 一个包含资源的结构体, 它实现了 Clone trait
    #[derive(Debug, Clone)]
    struct Pair(Box<i32>, Box<i32>);

    // 实例化 Nil
    let nil = Nil;

    // 复制 Nil 没有资源用于移动
    let copied_nil = nil;

    // 连个Nil 都可以独立使用
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // 实例化 Pair
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // 将 pair 绑定到 moved_pair 移动了资源
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // 报错 pair 已失去它的资源
    // println("original: {:?}", pair);

    // 将moved_pair (包括其资源) 克隆到 cloned_pair
    let cloned_pair = moved_pair.clone();
    //使用 std::mem::drop 来销毁原始 pair
    drop(moved_pair);

    // 报错 moved_pair 已被销毁
    // println("copy: {:?}", moved_pair);

    // 又.clone() 得来的结果任然可用
    println!("clone: {:?}", cloned_pair);

    // ============ 特征 trait -> 父 trait ==============
    trait Person {
        fn name(&self) -> String;
    }

    // Person 是 student 的父 trait
    // 实现Student需要你也impl 了 Person
    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    // CompSciStudent (computer science student 计算机可续的学生) 是Programmer 和 Student
    // 两者的子类, 实现 CompSciStudent 需要你同时 impl 了 两个父 trait
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}, My favorite language is {}, My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }
    // ============ 特征 trait -> 消除重叠 trait ==============
    trait UsernameWidget {
        // 从这个widget 中获取选定的用户名
        fn get(&self) -> String;
    }

    trait AgeWidget {
        // 从这个widget 中获取选定的年龄
        fn get(&self) -> u8;
    }
    // 同时具有UsernameWidget 和 AgeWidget 的表单
    struct Form {
        username: String,
        age: u8,
    }

    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }

    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }

    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // 如果取消注释慈航 , 则会收到一条错误消息, 提示multiple get found (找到多个 get)
    // 因为毕竟有很多名为 get 的方法
    // println("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);

    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
