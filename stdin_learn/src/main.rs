/// # 学习 Rust 中的标准输入
/// 首先我们要导入 stdin
/// `use std::io::stdin`

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
