#![allow(warnings, unused)]

use core::borrow;

fn main() {
    fn create_box() {
        // 在堆上分配一个整型数据
        let _box1 = Box::new(3i32);

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
    struct Person{
        name: String,
        age: u8,
    }
    let person = Person{
        name: String::from("Alice"),
        age: 20,
    };
    // name 从person 中移走但是 age 只是引用
    let Person{name, ref age} = person;
    println!("THe person's age is {}", age);
    println!("The person's name is {}", name);

    // 报错 部分移动值的借用, person 部分借用产生
    // println!("The person struct is {:?}", person);

    // person 不能使用 但是 person.age 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);

    // ===================借用
    // 此函数取得一个 box 的所有权炳销毁他
    fn eat_box_i32(boxed_i32: Box<i32>){
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
    struct Book{
        // &'static str 是一个对分配在只读内存区的字符串的引用
        author:&'static str,
        title:&'static str,
        year: u32,
    }
    // 此函数接受一个对 book 类型引用
    fn borrow_book(book:&Book) {
        println!("I immutabley borrowed {} - {} edittion", book.title, book.year);
    }

    // 此函数接受一个对可变的 book 类型的引用 他把年份 year 改为 2014 年
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edittion", book.title, book.year);
    }

    // 创建一个名为 immutabook 的不可变的book实例
    let immutabook = Book{
        author: "Douglas Hofstadter",
        title:"Godel, Escher, Bach",
        year:1979,
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

    struct Point{
        x:i32,
        y:i32,
        z:i32,
    }
    let mut point = Point{x:0, y:0, z:0};
    let borrowed_point = &point;
    let another_borrowed = &point;

    // 数据可以通过引用或原始类型来访问
    println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrowed.y, point.z);

    // 报错 point 不能以可变方式借用, 因为当前还有不可变借用
    // let mutable_borrow = &mut point;

    // 被即用的值在这里被重新使用
    println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrowed.y, point.z);

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
    println!("Point has coordinates: ({}, {}, {})", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // 可变引用不在用于其余的代码, 因此可以重新借用
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})", new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);

    //=====================作用域规则 -> ref 模式
    #[derive(Clone, Copy)]
    struct Point2{
        x:i32,
        y:i32,
    };
    let c = 'Q';
    
    // 赋值语句中左边的 ref 关键字等驾驭右边的&符号
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point2{x: 0, y:0};
    // 在解构一个结构体是 ref 同样有效
    let _copy_of_x = {
        // ref_to_x 是一个指向point 的x字段的引用
        let Point2 {x:ref ref_to_x, y:_} = point;
        *ref_to_x
    };
    // point 可变拷贝
    let mut mutable_point = point;
    {
        // ref 可以与 mut 结合以创建可变引用
        let Point2{x:_, y:ref mut mut_ref_to_y} = mutable_point;

        // 通过可变引用来改变 mutable_point 的字段 y
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutableZ_point is ({}, {})", mutable_point.x, mutable_point.y);

    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        // 解构  mutable_tuple 来改变last 的值
        let (_, ref mut  last ) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}
