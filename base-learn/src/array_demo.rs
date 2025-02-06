/// file: array_demo.rs
/// about: xiaotupo
/// email: xiaotupo@163.com
/// Date: 2025-2-4
/// Copyright (c) 2025, xiaotupo. All rights reserved.

pub fn run() {
    let iarr_1: [i32; 4] = [1, 2, 3, 4]; // 正常定义数组的方式
    println!("iarr_1: {:?}", iarr_1);

    // 3为初始值，4为数组长度，意思是创建一个长度为 4,所有元素的值都为 3
    let iarr_2 = [3; 4];
    println!("iarr_2: {:?}", iarr_2);

    let iarr_3: [i64; 4] = [11, 22, 33, 44];
    println!("iarr_3: {:?}", iarr_3);
}
