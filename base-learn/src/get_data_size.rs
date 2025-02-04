/// file: get_data_size.rs
/// about: xiaotupo
/// email: xiaotupo@163.com
/// Date: 2025-2-4
/// Copyright (c) 2025, xiaotupo. All rights reserved.

// 获取数据大小的 Example
pub fn run() {
    println!("i32 size: {} bytes", std::mem::size_of::<i32>());
    println!("i64 size: {} bytes", std::mem::size_of::<i64>());
    println!("char size: {} bytes", std::mem::size_of::<char>());
    println!("f32 size: {} bytes", std::mem::size_of::<f32>());
    println!("i64 size: {} bytes", std::mem::size_of::<i64>());

    let a: i32 = 0;
    let mut name = String::new();
    name.push_str("Hello");
    let b = [1, 2, 3];
    let f = 12.3;

    println!("a size: {} bytes", std::mem::size_of_val(&a));
    println!("name size: {} bytes", std::mem::size_of_val(&name));
    println!("b size: {} bytes", std::mem::size_of_val(&b));
    println!("f size: {} bytes", std::mem::size_of_val(&f));
}
