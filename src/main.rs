use anyhow::Result;
use clap::Parser;
use tracing::info;

mod daemon;
mod gateway;
mod sidecar;
mod utils;

// 命令行参数结构体
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(long_about = "\
Examples:
  rsweb -p 8080          # listen on port 8080
  rsweb --host 0.0.0.0   # bind to all interfaces")]
pub struct Gateway {
    /// 服务器绑定的IP地址
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// 服务器监听的端口
    #[arg(short, long, default_value_t = 41218)]
    port: u16,

    /// 日志级别 (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// 启用详细日志输出
    #[arg(short, long)]
    verbose: bool,

    /// 以守护进程模式运行
    #[arg(short, long)]
    daemon: bool,

    /// 守护进程模式下的 PID 文件路径
    #[arg(long, default_value = "/tmp/com.rsweb/rsweb.pid")]
    pid_file: String,

    /// 停止正在运行的守护进程
    #[arg(short, long)]
    stop: bool,
}

fn main() -> Result<()> {
    // 解析命令行参数
    let args = Gateway::parse();

    // 设置日志级别
    let log_level = if args.verbose {
        "debug"
    } else {
        &args.log_level
    };

    // 1. 处理停止守护进程
    if args.stop {
        utils::init_tracing(log_level);
        return daemon::stop_daemon(&args.pid_file);
    }

    // 2. 处理守护进程模式
    if args.daemon {
        // 在 daemonize 之前不初始化 tracing，避免 fork 后 FD 1/2 指向错误或 FD 泄露
        daemon::start_daemon(&args.pid_file)?;
    }

    // 在 fork (如果有) 之后初始化 tracing，确保日志输出到正确的地方 (stdout 或重定向的文件)
    utils::init_tracing(log_level);

    // 3. 初始化并运行 Tokio 运行时
    // 在 fork (daemonize) 之后启动运行时，避免 "Bad file descriptor" 错误
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(async {
        // 如果需要启动 Sidecar，可以在这里启动
        /*
        let sidecar = sidecar::Sidecar::new("my-sidecar-bin")?;
        tokio::spawn(async move {
            if let Err(e) = sidecar.run_and_log(&["--arg1"]).await {
                error!("Sidecar error: {}", e);
            }
        });
        */

        info!("🚀 Starting Listen on http://{}:{}", args.host, args.port);
        gateway::run_gateway(&args.host, args.port).await
    })
}
