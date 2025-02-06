/// file: string_demo.rs
/// about: xiaotupo
/// email: xiaotupo@163.com
/// Date: 2025-2-4
/// Copyright (c) 2025, xiaotupo. All rights reserved.

pub fn run() {
    let mut s = String::from("hello");
    println!("s: {s}");
    s.push_str(", world");
    println!("s: {s}");

    let s1 = s.clone();
    println!("s1: {s1}");
    println!("s: {s}");
}
