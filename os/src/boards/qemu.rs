//! QEMU板级支持模块
//!
//! 该模块提供了QEMU模拟器特定的功能，主要是退出机制
//! 参考：https://github.com/andre-richter/qemu-exit
use core::arch::asm; // 使用内联汇编

// 退出码常量
const EXIT_SUCCESS: u32 = 0x5555; // 等同于`exit(0)`，QEMU成功退出

const EXIT_FAILURE_FLAG: u32 = 0x3333; // 失败标志
const EXIT_FAILURE: u32 = exit_code_encode(1); // 等同于`exit(1)`，QEMU失败退出
const EXIT_RESET: u32 = 0x7777; // QEMU重置

/// QEMU退出接口
/// 定义了与退出相关的方法
pub trait QEMUExit {
    /// 使用指定的返回码退出
    ///
    /// 注意：对于`X86`，代码在QEMU内部与`0x1`进行二进制OR操作。
    fn exit(&self, code: u32) -> !;

    /// 使用`EXIT_SUCCESS`（即`0`）退出QEMU（如果可能）。
    ///
    /// 注意：对于`X86`不可用。
    fn exit_success(&self) -> !;

    /// 使用`EXIT_FAILURE`（即`1`）退出QEMU。
    fn exit_failure(&self) -> !;
}

/// RISCV64配置
/// 定义了RISC-V 64位架构的QEMU退出处理结构
pub struct RISCV64 {
    /// sifive_test映射设备的地址。
    addr: u64, // 退出设备的内存映射地址
}

/// 使用EXIT_FAILURE_FLAG编码退出码。
/// 将退出码左移16位并与EXIT_FAILURE_FLAG进行按位或操作
const fn exit_code_encode(code: u32) -> u32 {
    (code << 16) | EXIT_FAILURE_FLAG
}

impl RISCV64 {
    /// 创建一个RISCV64实例。
    pub const fn new(addr: u64) -> Self {
        RISCV64 { addr } // 初始化结构体
    }
}

/// 为RISCV64实现QEMUExit trait
impl QEMUExit for RISCV64 {
    /// 使用指定的退出码退出QEMU。
    fn exit(&self, code: u32) -> ! {
        // 如果代码不是特殊值，我们需要使用EXIT_FAILURE_FLAG进行编码。
        let code_new = match code {
            EXIT_SUCCESS | EXIT_FAILURE | EXIT_RESET => code, // 特殊值直接使用
            _ => exit_code_encode(code), // 其他值需要编码
        };

        unsafe {
            asm!(
                "sw {0}, 0({1})", // 将退出码写入指定地址
                in(reg)code_new, in(reg)self.addr // 将退出码和地址传入汇编指令
            );

            // 如果QEMU退出尝试不起作用，进入无限循环。
            // 这里不能调用`panic!()`，因为这个函数很可能就是在`panic!()`处理程序中被调用的。
            // 这样可以防止可能的无限循环。
            loop {
                asm!("wfi", options(nomem, nostack)); // 等待中断指令，让CPU进入低功耗状态
            }
        }
    }

    /// 成功退出QEMU
    fn exit_success(&self) -> ! {
        self.exit(EXIT_SUCCESS); // 调用exit函数并传入成功退出码
    }

    /// 失败退出QEMU
    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE); // 调用exit函数并传入失败退出码
    }
}

// QEMU虚拟机中测试设备的内存映射地址
const VIRT_TEST: u64 = 0x100000;

// 创建一个全局的QEMU退出处理器
pub const QEMU_EXIT_HANDLE: RISCV64 = RISCV64::new(VIRT_TEST);
