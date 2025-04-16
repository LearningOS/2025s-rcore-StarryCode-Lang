//! SBI控制台驱动，用于文本输出
//!
//! 该模块提供了基本的控制台输出功能，包括实现了print!和println!宏
use crate::sbi::console_putchar; // 导入SBI模块中的控制台输出字符函数
use core::fmt::{self, Write}; // 导入核心格式化相关功能

/// 标准输出结构体，用于实现Write trait
struct Stdout;

/// 为Stdout实现Write trait，使其能够输出格式化的文本
impl Write for Stdout {
    /// 实现write_str方法，将字符串输出到控制台
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 遍历字符串中的每个字符，并调用console_putchar输出
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(()) // 返回成功结果
    }
}

/// 打印格式化的参数到控制台
/// 这个函数被内部的print!和println!宏使用
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap(); // 使用Stdout结构体的write_fmt方法输出格式化参数
}

/// 使用格式化字符串和参数在宿主控制台上打印。
#[macro_export] // 将宏导出到包的根命名空间
macro_rules! print {
    // 宏规则：接受一个格式化字符串和可选的参数
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?)) // 调用print函数处理格式化参数
    }
}

/// 使用格式化字符串和参数在宿主控制台上打印，并在末尾添加换行符。
#[macro_export] // 将宏导出到包的根命名空间
macro_rules! println {
    // 宏规则：接受一个格式化字符串和可选的参数
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?)) // 调用print函数，并在格式化字符串后添加换行符
    }
}
