//! 主模块和入口点
//!
//! 操作系统和应用程序从这个模块开始。内核代码首先从`entry.asm`开始执行，
//! 然后调用[`rust_main()`]来初始化各种功能，如[`clear_bss()`]（查看其源代码获取详细信息）。
//!
//! 然后我们调用[`println!`]来显示`Hello, world!`。

#![deny(missing_docs)] // 拒绝缺少文档的代码
#![deny(warnings)] // 拒绝警告
#![no_std] // 不使用标准库
#![no_main] // 不使用标准main函数
#![feature(panic_info_message)] // 使用panic_info_message特性

use core::arch::global_asm; // 使用全局汇编宏
use log::*; // 使用日志功能

#[macro_use] // 使用宏
mod console; // 控制台模块
mod lang_items; // 语言项模块
mod logging; // 日志模块
mod sbi; // SBI调用模块

#[path = "boards/qemu.rs"] // 指定模块路径
mod board; // 板级支持模块

global_asm!(include_str!("entry.asm")); // 包含汇编入口文件

/// 清除BSS段
/// BSS段用于存储未初始化的全局变量，需要在启动时清零
pub fn clear_bss() {
    extern "C" {
        fn sbss(); // BSS段起始地址
        fn ebss(); // BSS段结束地址
    }
    // 将BSS段的每个字节清零
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// 操作系统的Rust入口点
#[no_mangle] // 不修改函数名称（避免编译器修改函数名）
pub fn rust_main() -> ! { // 返回类型!表示这个函数永不返回
    extern "C" {
        fn stext(); // 代码段起始地址
        fn etext(); // 代码段结束地址
        fn srodata(); // 只读数据段起始地址
        fn erodata(); // 只读数据段结束地址
        fn sdata(); // 数据段起始地址
        fn edata(); // 数据段结束地址
        fn sbss(); // BSS段起始地址
        fn ebss(); // BSS段结束地址
        fn boot_stack_lower_bound(); // 启动栈下界
        fn boot_stack_top(); // 启动栈顶部
    }
    clear_bss(); // 清除BSS段
    logging::init(); // 初始化日志系统
    println!("[kernel] Hello, world!"); // 打印欢迎信息
    // 使用不同级别的日志打印内存布局信息
    trace!(
        "[kernel] .text [{:#x}, {:#x})", // 打印代码段范围
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})", // 打印只读数据段范围
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})", // 打印数据段范围
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}", // 打印启动栈范围
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize); // 打印BSS段范围

    use crate::board::QEMUExit;
    crate::board::QEMU_EXIT_HANDLE.exit_success(); // CI自动测试成功
                                                   //crate::board::QEMU_EXIT_HANDLE.exit_failure(); // CI自动测试失败
}
