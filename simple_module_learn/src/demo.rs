#![allow(dead_code)] // 关闭所有未使用警告

use colored::Colorize;

#[derive(Default, Debug, Clone, Copy)]
pub struct Demo {
    a: i64,
    b: f64,
}

impl Demo {
    pub fn up_all(&mut self, a: i64, b: f64) {
        self.a = a;
        self.b = b;
    }

    pub fn print(demo: &Demo) {
        let left = format!("{}", "[".green().bold());
        let right = format!("{}", "]".green().bold());
        println!("{left}a: {}, b: {}{right}", demo.a, demo.b);
    }
}
