/// file: macros.rs
/// about: xiaotupo
/// email: xiaotupo@163.com
/// Date: 2025-2-4
/// Copyright (c) 2025, xiaotupo. All rights reserved.
/// 
/// ** 我的自定义宏 **
/// 

#[macro_export]
macro_rules! print_line {
    () => {
        println!("=============================================== {}", line!());
    };
}
