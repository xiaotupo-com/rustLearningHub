# 该项目演示了 Rust 中的简单模块使用方法

最简单的模块编程就是在 src 目录下建一个 .rs 文件，该文件就是一个模块，然后再在 main.rs 中使用，该项目用了一个第三方包 colored，包的作用是在终端可以输出彩色文字。

demo.rs:

```rust
#![allow(dead_code)] // 关闭所有未使用警告

use colored::Colorize;

#[derive(Default, Debug, Clone, Copy)]
pub struct Demo {
    a: i64,
    b: f64,
}

impl Demo {
    pub fn up_all(&mut self, a: i64, b: f64) {
        self.a = a;
        self.b = b;
    }

    pub fn print(demo: &Demo) {
        let left = format!("{}", "[".green().bold());
        let right = format!("{}", "]".green().bold());
        println!("{left}a: {}, b: {}{right}", demo.a, demo.b);
    }
}
```

main.rs

```rust
/// Rust 中的模块简单学习
/// 在 src 目录下新建一个 demo.rs，一个.rs文件就是一个模块，当然还有其他的模块方式

mod demo; // 先声明一下模块 demo

use demo::Demo;

fn main() {
    // 新建一个 Demo 对象
    let mut d1 = Demo::default();
    Demo::print(&d1); // 打印出的是 default() 生成的默认值
    // 更新 d1 对象
    d1.up_all(12, 24.23);
    Demo::print(&d1);

    // 这里克隆了一个 Demo 对象，因为我们为 Demo 实现了 Clone和Copy
    let mut d2 = d1;
    d2.up_all(10, 100.21); // 更新 d2 对象
    Demo::print(&d1);
    Demo::print(&d2);
}
```

## 总结

项目用到的素材：

1. `colored` 第三方模块，用于控制台可以输出多种颜色的内容
2. 用到的一些 `trait`：`Default`, `Debug`, `Clone`, `Copy`
3. 结构体关联函数，也就是以 `::`为前缀的函数
4. 结构体方法函数，也就是以 `.`前缀的函数
