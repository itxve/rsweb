use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod gateway;

// 命令行参数结构体
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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

    println!("🦀 Starting Listen on http://127.0.0.1:41111");

    gateway::run_gateway("127.0.0.1", 41111).await
}
