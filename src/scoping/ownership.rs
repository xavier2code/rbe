use std::fmt::{Display, Formatter};

pub fn destroy_box(c: Box<ToDrop>) {
    println!("Destroying a box thant contains {}", c);
}

pub fn ownership() {
    // 栈分配的整数
    let x = 5u32;
    // 这个复制不存在资源的移动
    let y = x;

    println!("x is {}, y is {}", x, y);

    // 堆分配的资源的指针
    let a = Box::new(ToDrop);
    //
    let b = a;
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
    // 同一个堆分配的数据，但是现在是 `b` 拥有它。

    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
    //println!("a contains: {}", a);
    // 试一试 ^ 去掉此行注释

    // 此函数从 `b` 中取得堆分配的内存的所有权
    destroy_box(b)
}

#[derive(Debug)]
pub struct ToDrop;

impl Display for ToDrop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ToDrop")
    }
}