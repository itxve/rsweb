use anyhow::Result;
use clap::Parser;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦀 Listen on http://127.0.0.1:41111");

    gateway::run_gateway("127.0.0.1", 41111).await
}
