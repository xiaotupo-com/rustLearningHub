/// file: main.rs
/// about: xiaotupo
/// email: xiaotupo@163.com
/// Date: 2025-2-4
/// Copyright (c) 2025, xiaotupo. All rights reserved.
mod array_demo;
mod get_data_size;
mod string_demo;
mod traits;
mod variables;
#[macro_use]
mod macros;

mod rectangle;

use get_data_size as get_size;
use rectangle::Rectangle;
use traits::print::Print;
use variables::Variable;

fn main() {
    let mut var = Variable::new();

    var.vfloat.set_vfloat(12.1, 23.3);
    var.vfloat.print();
    print_line!();

    var.vint.set_vint(12, 32, 49, 78, 1122);
    var.vint.print();
    print_line!();

    get_size::run();
    print_line!();

    array_demo::run();
    print_line!();

    string_to_number_demo();
    print_line!();

    number_to_string_demo();
    print_line!();

    tup_demo();
    print_line!();

    if_demo();
    print_line!();

    loop_demo();
    print_line!();

    while_demo();
    print_line!();

    for_demo();
    print_line!();

    string_demo::run();
    print_line!();

    let mut rec: Rectangle = Default::default();
    rec.set_height(12.55);
    rec.set_width(12.0);
    let area = rec.get_area();
    println!("area: {area}");
}

// String 转为数字类型
fn string_to_number_demo() {
    let size = "12.32";
    let size: f32 = size.parse().expect("转化时出错了");
    println!("size: {}", size);

    let size = size as i32;
    println!("size: {}", size);
}

// 数字转为 String类型
fn number_to_string_demo() {
    let num1 = 100;

    let str_num1 = num1.to_string();

    println!("str_num1: {}", str_num1);
}

fn tup_demo() {
    let tup = (500, 3.4, 22);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("x: {}, y: {}, z: {}", tup.0, tup.1, tup.2);
}

// 在 let 语句中使用 if
fn if_demo() {
    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("number: {number}");
}

fn loop_demo() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            continue; // 如果 counter 等于 10 则直接跳出当前循环
        }
        println!("counter: {counter}");
        if counter > 100 {
            break counter; // 跳出整个循环
        }
    };

    println!("result: {result}");
}

fn while_demo() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
}

fn for_demo() {
    let a = [1, 2, 3, 4, 5, 6, 7];
    for i in a {
        println!("i: {i}");
    }

    for number in (1..4).rev() {
        println!("number: {number}");
    }
}
