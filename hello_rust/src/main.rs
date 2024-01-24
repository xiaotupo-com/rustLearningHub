///# 这是我的第一个 Rust 程序
/// 该程序通过 `println!` 宏打印出一些信息
/// 这是 println! 宏的定义
/// ```
/// #[macro_export]
/// #[stable(feature = "rust1", since = "1.0.0")]
/// #[cfg_attr(not(test), rustc_diagnostic_item = "println_macro")]
/// #[allow_internal_unstable(print_internals, format_args_nl)]
/// macro_rules! println {
///     () => {
///         $crate::print!("\n")
///     };
///     ($($arg:tt)*) => {{
///         $crate::io::_print($crate::format_args_nl!($($arg)*));
///     }};
/// }
/// ```

fn main() {
    // println! 是一个带换行的宏，定义如下
    println!("1. Hello, world!");
    println!("2. Hello, {}", "World!");
    println!("3. {}, {}", "Hello", "World!");
    let world = "World!";
    println!("4. Hello, {world}");

    // println! 宏将锁定每个调用的标准输出。 如果您在热循环内调用 println!，则此行为可能是循环的瓶颈。 为了避免这种情况，用 io::stdout().lock() 锁定 stdout:
    use std::io::{stdout, Write};
    let mut lock = stdout().lock();
    writeln!(lock, "hello,world").unwrap();

    eprintln!("Error...");

}
