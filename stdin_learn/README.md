# Rust 标准输入实例

通过该项目实验 `rust` 的标准输入，类似 `c` 语言中的 `scanf()`。

该例子演示了两个功能：

1. 从终端读取内容到变量
2. 从文件读取内容到变量

## 从终端读取内容到变量

先看源码：

```rust
use std::io::stdin;

fn main() {
    println!("请输入一些内容个 hello 变量：");
    let mut hello = String::new();
    stdin().read_line(&mut hello).unwrap(); // 程序运行到这里会停住等你输入文字

    // 打印 hello 变量
    println!("variable hello>>>: {hello}");
}
```

首先我们要导入 stdin，因为不是每个程序到需要标准输入，所有 rust 默认没有给我们默认导入。

在这里我们用到的读取函数为 `read_line()`,这是一个成员函数，第一个参数为 `&self`，代表对象自己，不用管，第二个参数是 `&mut String`，所有我们要提供一个可变的 `String` 变量 `hello`。这样我们在终端输入的内容就会传输到 `hello` 中。

## 从文件读取内容到变量

源码：（这是修改过的最终源码）

```rust
use std::io::{self, stdin};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    println!("请输入一些内容个 hello 变量：");
    let mut hello = String::new();
    stdin().read_line(&mut hello)?; // 程序运行到这里会停住等你输入文字

    // 打印 hello 变量
    println!("variable hello>>>: {hello}");

    // 这里多提供一个例子
    // 这个例子是从文件读取内容到变量中去
    let mut f = File::open("foo.txt")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    // 打印 buffer 里读到的内容
    println!("buffer>>> {buffer}");

    Ok(())
}
```
