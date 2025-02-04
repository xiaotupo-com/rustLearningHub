/// file: print.rs
/// about: xiaotupo
/// email: xiaotupo@163.com
/// Date: 2025-2-4
/// Copyright (c) 2025, xiaotupo. All rights reserved.


pub trait Print {
    //* 打印对象 Trait 
    fn print(&self)
    where
        Self: std::fmt::Debug,
    {
        println!("{:#?}", self);
    }
}
