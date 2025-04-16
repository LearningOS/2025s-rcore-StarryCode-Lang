//! SBI调用包装器
//!
//! SBI (Supervisor Binary Interface) 是用于与底层固件或引导加载程序通信的接口

use core::arch::asm; // 使用内联汇编

const SBI_CONSOLE_PUTCHAR: usize = 1; // SBI控制台输出字符的功能号

/// 通用SBI调用函数
#[inline(always)] // 总是内联这个函数，避免函数调用开销
/// 参数：
/// - which: SBI调用的功能号
/// - arg0-arg2: 传递给SBI调用的参数
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "li x16, 0", // 设置x16寄存器为0
            "ecall",     // 触发环境调用，进入S模式
            inlateout("x10") arg0 => ret, // x10寄存器存放第一个参数和返回值
            in("x11") arg1,            // x11寄存器存放第二个参数
            in("x12") arg2,            // x12寄存器存放第三个参数
            in("x17") which,           // x17寄存器存放功能号
        );
    }
    ret // 返回结果
}

/// 使用SBI调用在控制台输出字符（通过QEMU的UART处理程序）
/// 参数：
/// - c: 要输出的字符的ASCII码
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0); // 调用SBI的控制台输出字符功能
}

use crate::board::QEMUExit; // 导入QEMU退出相关功能
/// 使用SBI调用来关闭内核
/// 返回类型：! 表示这个函数永不返回
pub fn shutdown() -> ! {
    crate::board::QEMU_EXIT_HANDLE.exit_failure(); // 使用QEMU退出处理程序来失败退出
}
