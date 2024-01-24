/// Rust 中的模块简单学习
/// 在 src 目录下新建一个 demo.rs，一个.rs文件就是一个模块，当然还有其他的模块方式

mod demo;

use demo::Demo;

fn main() {
    // 新建一个 Demo 对象
    let mut d1 = Demo::default();
    Demo::print(&d1); // 打印出的是 default() 生成的默认值
    // 更新 d1 对象
    d1.up_all(12, 24.23);
    Demo::print(&d1);

    let mut d2 = d1;
    d2.up_all(10, 100.21);
    Demo::print(&d1);
    Demo::print(&d2);
}
