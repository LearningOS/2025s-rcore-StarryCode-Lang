//! 异常处理程序
//!
//! 该模块实现了Rust语言需要的panic处理程序，当程序发生严重错误时会被调用

use crate::sbi::shutdown; // 导入关机函数
use core::panic::PanicInfo; // 导入panic信息结构体

#[panic_handler] // 标记这个函数为异常处理程序
/// panic处理程序
/// 当程序发生严重错误时，这个函数会被调用
fn panic(info: &PanicInfo) -> ! { // 返回类型!表示这个函数永不返回
    // 如果可以获取到异常发生的位置信息
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}", // 打印异常位置和信息
            location.file(), // 文件名
            location.line(), // 行号
            info.message().unwrap() // 异常信息
        );
    } else {
        // 如果无法获取位置信息，只打印异常信息
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown() // 关闭系统
}
