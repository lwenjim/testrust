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
}
