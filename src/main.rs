use scoping::raii::create_multiple_box as cb;
use scoping::ownership::ownership as os;
use scoping::borrowing::borrowing as br;
use scoping::mutability::mutability as mb;

mod scoping;

fn main() {
    // 资源获取即初始化, 不用担心内存泄漏
    cb();
    // 所有权和移动
    os();
    // 借用
    br();
    // 可变性
    mb();
}
