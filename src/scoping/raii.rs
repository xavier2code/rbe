fn create_box() {
    let _box1 = Box::new(ToDrop);
}

pub fn create_multiple_box() {
    for _ in 0u32..1 {
        create_box()
    }
    // 完全不会泄漏
}

struct ToDrop;

// 析构函数
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped")
    }
}