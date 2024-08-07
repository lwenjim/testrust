#![allow(warnings, unused)]
fn main() {
    use std::mem;
    let color = String::from("green");
    let print = || println!("color:{}", color);
    // 使用借用来调用闭包
    print();

    // color 可再次被不可变借用, 因为闭包支持有一个指向color的不可变引用
    let _reborrow = &color;
    print();

    // 在最后使用 print 后 移动或者重新借用都是允许的
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 count 值增加, 要做到这点, 他需要得到&mut count 或者 count本身 但是 &mut count 的要求
    // 没那么严格, 所以我们采用这种方式
    // 该闭包立即借用 count
    // inc 前面需要加上mut  因为闭包里面存储着一个&mut 变量, 调用闭包时 该变量的变化就意味着闭包内部发生了变化
    // 因此闭包需要时可变的
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包, 所以仍然可变借用 count 视图重新借用将导致错误
    // let _reborrow = &count;
    // 试一试讲慈航去掉注释
    inc();

    // 闭包不再借用 &mut count 因此可以正确地重新借用
    let _count_reborrowed = &mut count;
    // 不可复制类型 (non-copy type)
    let movalbe = Box::new(3);

    // mem::Drop 要求 T 类型本身 所以闭包讲会捕获变量的值, 这种情况下
    // 可复制类型将会复制给闭包,从而原始值不受影响, 不可复制类型必须移动(move) 到闭包中
    // 因而 moveable 变量在这里立即移动到了闭包中
    let consume = || {
        println!("moveable: {:?}", movalbe);
        mem::drop(movalbe);
    };
    consume();

    // consume();

    // 在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权：
    // Vec 在语义上是不可复制的
    let haystack = vec![1, 2, 3];
    let contains = |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There {} elements in vec", haystack.len());
    // 取消上面一行的注释将导致编译时错误, 因为借用检查不允许在变量移动走以后继续使用它
    // 在闭包的签名中删除 move 会到值闭包已不可变方式借用haystack 因此之后
    // haystack 仍然可用, 取消上面的注释也不会导致错误

    // ===========作为输入参数============
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }
    let greeting = "hello";
    // 不可复制类型
    // to_owned 从借用的数据创建所有权的数据
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        // greeting 通过引用捕获 故需要闭包时 Fn
        println!("I sail{}.", greeting);
        //下文改变了 forewell 因而要求闭包通过可变引用捕获它
        // 现在需要FnMut
        farewell.push_str("!!!");
        println!("Then I scremed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用drop 又要求闭包通过值获取 farewell
        // 现在需要FnOnce
        mem::drop(farewell);
    };
    apply(diary);

    // 闭包 double 满足 apply_to_3 的trait 约束
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));

    use std::mem;
    let color = String::from("green");
    let print = || println!("color:{}", color);
    // 使用借用来调用闭包
    print();

    // color 可再次被不可变借用, 因为闭包支持有一个指向color的不可变引用
    let _reborrow = &color;
    print();

    // 在最后使用 print 后 移动或者重新借用都是允许的
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 count 值增加, 要做到这点, 他需要得到&mut count 或者 count本身 但是 &mut count 的要求
    // 没那么严格, 所以我们采用这种方式
    // 该闭包立即借用 count
    // inc 前面需要加上mut  因为闭包里面存储着一个&mut 变量, 调用闭包时 该变量的变化就意味着闭包内部发生了变化
    // 因此闭包需要时可变的
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包, 所以仍然可变借用 count 视图重新借用将导致错误
    // let _reborrow = &count;
    // 试一试讲慈航去掉注释
    inc();

    // 闭包不再借用 &mut count 因此可以正确地重新借用
    let _count_reborrowed = &mut count;
    // 不可复制类型 (non-copy type)
    let movalbe = Box::new(3);

    // mem::Drop 要求 T 类型本身 所以闭包讲会捕获变量的值, 这种情况下
    // 可复制类型将会复制给闭包,从而原始值不受影响, 不可复制类型必须移动(move) 到闭包中
    // 因而 moveable 变量在这里立即移动到了闭包中
    let consume = || {
        println!("moveable: {:?}", movalbe);
        mem::drop(movalbe);
    };
    consume();

    // consume();

    // 在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权：
    // Vec 在语义上是不可复制的
    let haystack = vec![1, 2, 3];
    let contains = |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There {} elements in vec", haystack.len());
    // 取消上面一行的注释将导致编译时错误, 因为借用检查不允许在变量移动走以后继续使用它
    // 在闭包的签名中删除 move 会到值闭包已不可变方式借用haystack 因此之后
    // haystack 仍然可用, 取消上面的注释也不会导致错误

    // ===========作为输入参数============
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }
    let greeting = "hello";
    // 不可复制类型
    // to_owned 从借用的数据创建所有权的数据
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        // greeting 通过引用捕获 故需要闭包时 Fn
        println!("I sail{}.", greeting);
        //下文改变了 forewell 因而要求闭包通过可变引用捕获它
        // 现在需要FnMut
        farewell.push_str("!!!");
        println!("Then I scremed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用drop 又要求闭包通过值获取 farewell
        // 现在需要FnOnce
        mem::drop(farewell);
    };
    apply(diary);

    // 闭包 double 满足 apply_to_3 的trait 约束
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FuMut".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_FnOnce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_FnOnce();

    fn_plain();
    fn_mut();
    fn_once();

    // ================Iterator::any=================
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    // =================Iterator::find==================
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();
    // 对迭代器举出的元素的引用就是 &&i32 类型, 解构成 i32类型
    // 注意 find 方法会把迭代器元素的引用传递给闭包, 迭代器元素自身是 &i32 类型 所以传递给闭包的类型是 &&i32 类型
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    // ======================高阶函数=========================
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    println!("Find the num of all the squared odd numbers under 1000");
    let upper = 1000;
    // 命令式 imperative 的写法
    // 声明累加变量
    let mut acc = 0;
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;
        if n_squared >= upper {
            // 若大于上限则退出循环
            break;
        } else if is_odd(upper) {
            // 如果是奇数就计数
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("functional style: {}", sum_of_squared_odd_numbers);

    // =======================发散函数==========================
    fn foo() -> ! {
        panic!("This call never returns.");
    }

    fn some_fn() {
        ()
    }

    let a: () = some_fn();
    println!("This function returns and you can see this line.");

    // let x : ! = panic!("This call never returns.");
    // println!("You will never see this line!");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个match 表达式的返回值必须是u32
            // 因为addition变量是这个类型
            let addition: u32 = match i % 2 == 1 {
                // i 变量的类型为u32 这毫无问题
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    // =====================可见性=======================
    // 模块机制消除了相同名称的项之间的歧义
    function();
    my_mod::function();

    // 公有项, 包括嵌套模块内的, 都可以在父模块外部访问
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个crate中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错 函数 public_function_in_my_mod 是私有的
    // my_mod::nested::public_function()_in_my_mod();
    // 试一试取消该行的注释

    // 模块的私有项不能直接访问, 即便他是嵌套在公有模块内部的
    // 报错 private_function 是私有的
    // my_mod::private_function();
    // 试一试取消此行的注释

    // 报错private_function 是私有的
    // my_mod::nested::private_function();
    // 试一试 取消慈航的注释

    // error private_nested is a private module
    // my_mod::private_nested::function();
    // 试一试 取消此行的注释

    // ==================结构体的可见性=========================
    // 带有公有字段的公有结构体, 可以项平常一样构造
    let open_box = my::OpenBox {
        contents: "public information",
    };

    // 并且他们的字段可以正常访问到
    println!("The open box contains: {}", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名称构造
    // 报错 closeBox 含有私有字段
    // let closed_box = my::ClosedBox{contents: "classified information"};
    // 试一试 取消此行注释

    // 不过带有私有字段的结构体可以使用公有的构造器来构造
    let _closed_box = my::ClosedBox::new("classified information");

    // 并且一个结构体中地上有字段不能访问到
    // 报错 content 字段是私有的
    // println("The closed box contains: {}", _closed_box.contents);
    // 试一试 取消此行注释

    // ==================use 声明=========================
    use deeply::nested::function as other_function;

    // 更容易访问 deeply::nested::function
    other_function();

    println!("Entering block");
    {
        // 这和 use deeply::nested::function as function 等价
        // 此function() 将这遮蔽外部的同名函数
        use deeply::nested::function;
        function();

        // use 绑定拥有局部作用域, 在这个例子中 function()
        // 的遮蔽只存在这个代码快块中
        println!("Leaving block");
    }
    function();

    my2::indirect_call();

    

    // extern crate rary;
    rary::public_function();

    // 报错private_function 是私有的
    // rary::private_function();

    rary::indirect_access();
}

fn function3() {
    println!("called function3()");
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

mod my2 {
    fn function() {
        println!("called my::function()");
    }
    mod cool {
        pub fn function() {
            println!("called my::cool::function()");
        }
    }

    pub fn indirect_call() {
        // 让我们从这个作用域中访问所有名为 function 的函数
        print!("called my::indirect_call(). that\n>");

        // self 关键字表示当前的模块作用域--在这个例子 my
        // 调用 self::function()  和 直接调用function() 都得到相同的结果
        // 因为他们表示相同的函数
        self::function();
        function();

        // 我们也可以使用 self 来访问 my 内部的另一个模块
        self::cool::function();

        // super 关键字表示父作用域 (在my模块外面)
        super::function3();

        // 这将在*crate*作用域内绑定 cool::function
        // 在这个例子中, crate 作用与时嘴歪面的作用域
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }

}

fn function2() {
    println!("called function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()")
        }
    }
}

mod my {
    // 一个公有的结构体 带有一个公有的字段, (类型为泛型T)
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个公有的结构体, 带有一个私有的字段, (类型为泛型T)
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 一个公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called my_mod::private_function()");
    }

    // 使用 pub 修饰语句改变默认可见性
    pub fn function() {
        println!("called my_mod::function()");
    }

    // 同一个模块中, 项可以访问其他项, 及时它是私有的
    pub fn indirect_access() {
        print!("called my_mod::indirect_access(), that");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called my_mod::nested::private_function()");
        }

        // 使用 pub(in path) 语法定义的函数只在给定的路径中可见
        // path必须是父模块, (parent module) 或祖先模块(ancestor module)
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called my_mod::nested::public_function_in_my_mod(), that\n > ");
            public_function_in_nested()
        }

        // 使用pub(self) 语法定义的函数则只在前模块中可见
        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested");
        }

        // 使用 pub(super) 语法定义的函数只在父模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called my_mod::call_public_function_in_my_mod(), that\n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) 使得函数只在当前crate 中可见
    pub(crate) fn public_function_in_crate() {
        println!("called my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called my_mod::private_nested::function()");
        }
    }
}

fn function() {
    println!("called function()");
}
