#![allow(warnings, unused)]
use core::{borrow, fmt::Debug};

fn main() {
    fn create_box() {
        // 在堆上分配一个整型数据
        let _box1 = Box::new(3i32);
        use std::mem;
        let color = String::from("green");
        let print = || println!("color:{}", color);
        // 使用借用来调用闭包
        print();

        // _box1 在这里被销毁, 内存得到释放
    }

    // 在堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    // 嵌套作用域
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4i32);

        // _box3 在这里被销毁, 内存得到释放
    }

    // 创建一大堆 box (只是因为好玩)
    // 完全不需要手动释放内存
    for _ in 0u32..1_000 {
        create_box();
    }

    // _box2 在这里被销毁, 内存得到释放
    struct ToDrop;
    impl Drop for ToDrop {
        fn drop(&mut self) {
            //println!("ToDrop is being dropped!");
        }
    }

    let x = ToDrop;
    println!("Made a ToDrop");

    // ===================所有权和移动=====================
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);

        // c 被销毁且内存得到释放
    }

    // 栈分配的整型
    let x = 5u32;

    // 将 x, 复制到 y 不存在资源移动
    let y = x;

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // a 是一个指向堆分配的证书的指针
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // 移动 a 到 b
    let b = a;

    // 把a的指针(而非数据)复制到b, 现在两者都是指向
    // 同一个堆分配的数据, 但是现在是 b 拥有他

    // 报错 a 不能访问数据, 因为他不在拥有那部分对上的内存
    // println("a contians: {}", a);

    // 此函数从 b 中提取堆分配的内存的所有者
    destroy_box(b);

    // 此时堆内存已经释放, 这个操作会导致解引用释放内存, 而这是编译器禁止的
    // 报错 和 前面出错的原因一样
    // println("b contains: {}", b);

    // =========可变性==========
    // 当所有权转移时, 数据的可变性可能发生改变
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains: {}", immutable_box);

    // 可变性错误
    // *immutable_box = 4;

    // 移动 box 改变所有权  (和可变性)
    let mut mutable_box = immutable_box;
    println!("mutable_box contains: {}", mutable_box);

    *mutable_box = 4;
    println!("mutable_box now conntains {}", mutable_box);

    //======部分移动
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };
    // name 从person 中移走但是 age 只是引用
    let Person { name, ref age } = person;
    println!("THe person's age is {}", age);
    println!("The person's name is {}", name);

    // 报错 部分移动值的借用, person 部分借用产生
    // println!("The person struct is {:?}", person);

    // person 不能使用 但是 person.age 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);

    // ===================借用
    // 此函数取得一个 box 的所有权炳销毁他
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    // 此函数借用了一个 i32 类型
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用了box的内容, 但没有取得所有权, 所以 box的内容之后可以再次借用,
    // 请注意函数资深就是一个作用域, 因此下面两个函数运行完成以后
    // 在函数中临时创建的引用也就不复存在了
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        // 取得一个对 box 中数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 报错
        // 当 boxed_i32 里面的值之后在作用域中被借用时, 不能将其销毁
        // eat_box_i32(boxed_i32);

        // 在 _ref_to_i32 里面的值被销毁后, 尝试借用 _ref_to_i32
        // 如果此处不借用, 则在上一行的代码中, eat_box_i32(boxed_i32)可以讲 boxed_i32 销毁
        borrow_i32(_ref_to_i32);

        println!("{}", boxed_i32);
    }

    // ======================作用域规则->可变性
    #[derive(Clone, Copy)]
    struct Book {
        // &'static str 是一个对分配在只读内存区的字符串的引用
        author: &'static str,
        title: &'static str,
        year: u32,
    }
    // 此函数接受一个对 book 类型引用
    fn borrow_book(book: &Book) {
        println!(
            "I immutabley borrowed {} - {} edittion",
            book.title, book.year
        );
    }

    // 此函数接受一个对可变的 book 类型的引用 他把年份 year 改为 2014 年
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edittion", book.title, book.year);
    }

    // 创建一个名为 immutabook 的不可变的book实例
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // 创建一个 immutabook 的可变拷贝 命名为 mutabook
    let mut mutabook = immutabook;

    // 不可变的借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变的借用一个可变对象
    borrow_book(&mutabook);

    // 可变借用一个可变对象
    new_edition(&mut mutabook);

    // 报错 不能可变的借用一个不可变对象
    // new_edition(&mut immutabook);

    // ================作用域规则-> 别名使用

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let mut point = Point { x: 0, y: 0, z: 0 };
    let borrowed_point = &point;
    let another_borrowed = &point;

    // 数据可以通过引用或原始类型来访问
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrowed.y, point.z
    );

    // 报错 point 不能以可变方式借用, 因为当前还有不可变借用
    // let mutable_borrow = &mut point;

    // 被即用的值在这里被重新使用
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrowed.y, point.z
    );

    // 不可变的引用不在用于其余代码, 因此可以使用可变的引用重新借用
    let mutable_borrow = &mut point;

    // 通过可变引用来修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 报错 不能再以不可变的方式来节约 point 因为他当前已经被可变借用
    // let y = &point.y;

    // 报错 无法打印 因为println 用到了一个不可变引用
    // println("Point z coordinate is {}", point.z);

    // 正常运行 可变引用能够以不可变类型传入 println
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // 可变引用不在用于其余的代码, 因此可以重新借用
    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );

    //=====================作用域规则 -> ref 模式
    #[derive(Clone, Copy)]
    struct Point2 {
        x: i32,
        y: i32,
    };
    let c = 'Q';

    // 赋值语句中左边的 ref 关键字等驾驭右边的&符号
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point2 { x: 0, y: 0 };
    // 在解构一个结构体是 ref 同样有效
    let _copy_of_x = {
        // ref_to_x 是一个指向point 的x字段的引用
        let Point2 {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };
    // point 可变拷贝
    let mut mutable_point = point;
    {
        // ref 可以与 mut 结合以创建可变引用
        let Point2 {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        // 通过可变引用来改变 mutable_point 的字段 y
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutableZ_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        // 解构  mutable_tuple 来改变last 的值
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);

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
        std::mem::drop(movalbe);
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
        std::mem::drop(farewell);
    };
    apply(diary);

    // 闭包 double 满足 apply_to_3 的trait 约束
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
    let a = [1, 2, 3];

    // =================生命周期==================
    // print_refs 接受两个 i32 的引用 他们有不同的生命周期 `a 和 `b
    // 这两个生命周期必须至少要和print_refs 函数一样长
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    // 不带参数的函数, 不过有一个生命周期参数
    fn failed_borrow<'a>() {
        let _x = 12;
        // 报错 _x 的生命周期不够长
        // let y:&'a i32 = &_x;
        // 在函数内部使用生命周期 'a 作为显示类型标注将导致失败, 因为 &_x 的
        // 声明周期比 y 的短, 短声明周期不能强制转成长生命周期
    }

    // 创建变量, 稍后用于借用
    let (four, nine) = (4, 9);

    // 两个变量的借用, (&) 都是传递函数
    print_refs(&four, &nine);
    // 任何被借用的输入量都必须比借用这生存得更长
    // 也就是说 four 和 nine 的生命周期都必须比 print_refs 的长

    failed_borrow();
    // failed_borrow 未包含引用 因此不要求, 'a 长于函数的生命周期
    // 但 'a 寿命确实更长, 因为该生命周期从未被约束, 所以默认为 'static

    // ==================生命周期--> 函数
    // 一个拥有生命周期 'a 的输入引用 其中 'a 的存活时间
    // 至少与函数的一样长
    fn print_one<'a>(x: &'a i32) {
        println!("print_one: x is {}", x);
    }

    // 可变引用同样也可能拥有生命周期
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    // 拥有不同生命周期的多个元素, 对下面这种情形, 两者即时拥有
    // 相同的生命周期 'a 也没问题, 但对一些更复杂的情形,可能
    // 就需要不同的生命周期了
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("print_multi : x is {}, y is {}", x, y);
    }

    // 返回传递进来的引用也是可行的
    // 但必须返回争取的生命周期
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    // fn invalid_output<'a>() -> &'a String {&String::from("foo")}
    // 上面代码是无效的, 'a存活时间必须比函数的长
    // 这里的 &String::from("foo") 将会创建一个 String类型 然后怼他取引用

    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    // ========生命周期 -> 方法
    struct Owner(i32);

    impl Owner {
        // 标注生命周期, 就像独立的函数一样
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }
        fn print<'a>(&'a self) {
            println!("print: {}", self.0);
        }
    }
    let mut owner = Owner(18);
    owner.add_one();
    owner.print();

    // =============生命周期-> 结构体
    // 一个 borrowed 类型 含有一个指向 i32 类型的引用
    // 该引用必须比 borrowed 寿命更长
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // 和前面类似 这里的两个音乐都必须比这个接头体长寿
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    // 一个枚举类型,  其取值不是 i32 类型就是一个指向 i32 的引用
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }
    let x = 18;
    let y = 15;
    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

    // ===============生命周期-> trait=========
    // trait 方法中生命期的标注基本上与函数类似。注意，impl 也可能有生命周期的标注。
    // 带有生命周期标注的结构体
    #[derive(Debug)]
    struct Borrowed2<'a> {
        x: &'a i32,
    }

    // 给impl 标注声明周期
    impl<'a> Default for Borrowed2<'a> {
        fn default() -> Self {
            Self { x: &10 }
        }
    }
    let b: Borrowed2 = Default::default();
    println!("b is {:?}", b);

    // =================作用域-> 约束 ==============
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);
    // Ref 包含一个指向泛型类型 T的引用 其中T拥有一个未知的生命周期
    // 'a T 拥有生命周期限制, T 中的任何 引用都必须比'a获得更长 ,另外
    // Ref 的生命周期也不能超出'a

    // 一个泛型函数, 使用Debug trait 来打印内容
    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("print: t is {:?}", t);
    }

    // 这里接受一个指向 T 的引用 其中T实现了 Debug trait, 并且T中所有引用都必须比
    // 'a 获得时间更长, 另外 'a 也要比函数获得更长
    fn print_ref<'a, T>(t: &'a T)
    where
        T: Debug + 'a,
    {
        println!("print_ref: t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);

    // ==================作用域-> 强制转换=============
    // 在这里 rust 推到了一个尽可能短的生命周期
    // 然后这两个引用都被强制转成这个生命周期
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    // <'a: 'b, 'b> 读作生命周期 'a 至少和 'b 一样长
    // 在这里我们接受了一个 &'a i32 类型并返回一个&'b i32 类型 这是
    // 强制转换得到的结果
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }
    let first = 2; // 较长的生命周期
    {
        let second = 3; // 较短生命周期
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }

    //======================作用域->static==============
    // 产生一个拥有 'static 生命周期的常量
    static NUM: i32 = 18;

    // 返回一个指向NUM的引用 该引用不取 NUM的'static生命周期
    // 而是被强制转换成和输入参数一样的
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    {
        // 产生一个 String 字面量并打印他
        let static_string = "I'm in red-only memory";
        println!("static_string: {}", static_string);

        // 当static_String 离开作用域时, 该引用不能再使用, 不过
        // 数据任然存在二进制文件里面
    }
    {
        // 产生一个整型给 coerce_static 使用
        let lifetime_num = 9;
        // 将对NUM 的引用强制转成lifetime_num 的生命周期
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible!", NUM);

    // =======================作用域-> 省略 ===========
    // `elided_input` 和 `annotated_input` 事实上拥有相同的签名，
    // `elided_input` 的生命周期会被编译器自动添加：
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x)
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x)
    }

    // 类似地，`elided_pass` 和 `annotated_pass` 也拥有相同的签名，
    // 生命周期会被隐式地添加进 `elided_pass`：
    fn elided_pass(x: &i32) -> &i32 {
        x
    }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
