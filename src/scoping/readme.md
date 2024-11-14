# Rust中的作用域

## RAII
> src/scoping/raii.rs

使用 Box<T> 在堆上创建资源，使用自己析构函数实现 `Drop` trait，任何对象在离开作用域
时，它的析构函数（destructor）就被调用，然后它占有的资源就被释放。

> src/scoping/ownership.rs

如果是在栈上创建的资源，复制不会移动所有权，如果是通过 `Box::new(T)` 创建在堆上的指针，复制的时候会移交所有权，原来的变量就失去了所有权

> src/scoping/borrowing.rs

可以使用引用 `&T`，保护所有权不被拿走