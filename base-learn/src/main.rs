/// file: main.rs
/// about: xiaotupo
/// email: xiaotupo@163.com
/// Date: 2025-2-4
/// Copyright (c) 2025, xiaotupo. All rights reserved.

mod variables;
mod traits;
mod get_data_size;
mod array_demo;

#[macro_use]
mod macros;

use get_data_size as get_size;
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

}

fn string_to_number_demo(){
    let size = "12";
    let size: i32 = size.parse().expect("转化时出错了");
    println!("size: {}", size);
}
