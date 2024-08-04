use List::*;
enum List {
    // Cons 元组结构体, 包含连表的一个元素和指向下一个节点的指针
    Cons(u32, Box<List>),
    Nil,
}
impl List {
    // 创建一个空的List实例
    fn new() -> List {
        // nil 为 list 类型
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // format 和 print 类似 但返回的是一个对分配的字符串
                // 而不是打印结果到控制台
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示连表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
