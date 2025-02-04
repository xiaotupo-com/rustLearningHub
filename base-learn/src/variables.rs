/// file: variables.rs
/// about: xiaotupo
/// email: xiaotupo@163.com
/// Date: 2025-2-4
/// Copyright (c) 2025, xiaotupo. All rights reserved.


use crate::traits::print;

// Variable 结构体定义
pub struct Variable {
    pub vint: Vint,
    pub vfloat: VFloat,
}

/// VFloat 结构体定义
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct VFloat {
    vf32: f32,
    vf64: f64,
}

/// Vint 结构体定义
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Vint {
    vi8: i8,
    vi16: i16,
    vi32: i32,
    vi64: i64,
    vi128: i128,
}

// Variable 实现
impl Variable {
    pub fn new() -> Variable {
        Self {
            vint: Default::default(),
            vfloat: Default::default(),
        }
    }
}

/// Vint 实现
impl Vint {
    // 设置值
    pub fn set_vint(&mut self, vi8: i8, vi16: i16, vi32: i32, vi64: i64, vi128: i128) {
        self.vi8 = vi8;
        self.vi16 = vi16;
        self.vi32 = vi32;
        self.vi64 = vi64;
        self.vi128 = vi128;
    }
}

/// VFloat 实现
impl VFloat {
    // 设置值
    pub fn set_vfloat(&mut self, vf32: f32, vf64: f64) {
        self.vf32 = vf32;
        self.vf64 = vf64;
    }
}

// 实现 Print Trait
impl print::Print for VFloat {}
impl print::Print for Vint {}
