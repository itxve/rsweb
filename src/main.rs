use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};

mod gateway;

// 命令行参数结构体
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(long_about = "\
Examples:
  rsweb -p 8080          # listen on port 8080
  rsweb --host 0.0.0.0   # bind to all interfaces")]
struct Gateway {
    /// 服务器绑定的IP地址
    #[arg(short, long, default_value = "0.0.0.0")]
    host: String,

    /// 服务器监听的端口
    #[arg(short, long, default_value_t = 43218)]
    port: u16,

    /// 日志级别 (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// 启用详细日志输出
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let args = Gateway::parse();

    // 设置日志级别
    let log_level = if args.verbose {
        "debug"
    } else {
        &args.log_level
    };

    init_tracing(log_level);

    info!("🚀 Starting Listen on http://{}:{}", args.host, args.port);

    gateway::run_gateway(&args.host, args.port).await
}

// 初始化 tracing 的函数
fn init_tracing(log_level: &str) {
    // 方法1: 最简单的初始化 - 通常这就够了
    tracing_subscriber::fmt()
        .with_max_level(level_from_str(log_level))
        .with_target(true) // 显示日志目标
        .with_thread_ids(true) // 显示线程ID
        .with_file(true) // 显示文件名
        .with_line_number(true) // 显示行号
        .init();

    // 或者使用方法2: 更灵活的配置
    /*
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(level_from_str(log_level))
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
    */

    // 或者使用方法3: 支持环境变量
    /*
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(log_level))
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
    */

    info!("✅ Tracing initialized with level: {}", log_level);
}

// 将字符串转换为 Level
fn level_from_str(level: &str) -> Level {
    match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    }
}
