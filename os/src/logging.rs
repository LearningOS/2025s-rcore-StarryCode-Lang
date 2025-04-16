//! 全局日志系统
//!
//! 该模块实现了一个简单的日志系统，用于输出不同级别的日志信息

use log::{Level, LevelFilter, Log, Metadata, Record}; // 导入日志相关类型

/// 一个简单的日志器
/// 实现了Log trait，用于处理日志记录
struct SimpleLogger;

/// 为SimpleLogger实现Log trait
impl Log for SimpleLogger {
    /// 检查日志是否启用
    /// 这里始终返回true，表示所有日志都启用
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    /// 记录日志
    /// 根据日志级别使用不同的颜色输出
    fn log(&self, record: &Record) {
        // 如果日志级别未启用，直接返回
        if !self.enabled(record.metadata()) {
            return;
        }
        // 根据日志级别选择不同的颜色
        let color = match record.level() {
            Level::Error => 31, // 红色
            Level::Warn => 93,  // 亮黄色
            Level::Info => 34,  // 蓝色
            Level::Debug => 32, // 绿色
            Level::Trace => 90, // 亮黑色
        };
        // 使用ANSI转义序列设置颜色并打印日志
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m", // 使用ANSI转义序列设置颜色
            color,                         // 颜色代码
            record.level(),                // 日志级别
            record.args(),                 // 日志内容
        );
    }

    /// 刷新日志
    /// 这里不需要实现任何功能
    fn flush(&self) {}
}

/// 初始化日志系统
pub fn init() {
    // 创建一个静态的日志器实例
    static LOGGER: SimpleLogger = SimpleLogger;
    // 设置全局日志器
    log::set_logger(&LOGGER).unwrap();
    // 根据环境变量LOG设置日志级别
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error, // 只显示错误
        Some("WARN") => LevelFilter::Warn,   // 显示警告和错误
        Some("INFO") => LevelFilter::Info,   // 显示信息、警告和错误
        Some("DEBUG") => LevelFilter::Debug, // 显示调试信息及以上级别
        Some("TRACE") => LevelFilter::Trace, // 显示所有日志
        _ => LevelFilter::Off,              // 默认关闭日志
    });
}
