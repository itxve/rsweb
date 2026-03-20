use anyhow::Result;
use std::{fs::File, process::Command};
use tracing::info;

pub fn start_daemon(pid_file: &str) -> Result<()> {
    // 检查是否已经启动
    if let Ok(pid_str) = std::fs::read_to_string(pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            // 使用 kill -0 探测进程是否存在
            let status = Command::new("kill").arg("-0").arg(pid.to_string()).status();
            if let Ok(s) = status {
                if s.success() {
                    eprintln!("Server is already running (PID: {}).", pid);
                    std::process::exit(1);
                }
            }
        }
    }

    #[cfg(unix)]
    {
        use daemonize::Daemonize;

        // 确保 PID 文件所在目录存在
        if let Some(parent) = std::path::Path::new(pid_file).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let dir = std::path::Path::new(pid_file)
            .parent()
            .unwrap_or(std::path::Path::new("/tmp"));
        let stdout = File::create(dir.join(format!("{}.out", env!("CARGO_PKG_NAME"))))?;
        let stderr = File::create(dir.join(format!("{}.err", env!("CARGO_PKG_NAME"))))?;

        let daemonize = Daemonize::new()
            .pid_file(pid_file) // 指定 PID 文件
            .working_directory(dir) // 工作目录
            .stdout(stdout) // 重定向 stdout
            .stderr(stderr); // 重定向 stderr

        match daemonize.start() {
            Ok(_) => {
                // info!("Success, daemonized"); // 这里不再打印，交给 main 函数初始化后再打印
                Ok(())
            }
            Err(e) => {
                eprintln!("Error, {}", e);
                std::process::exit(1);
            }
        }
    }

    #[cfg(windows)]
    {
        // Windows 下暂不支持 daemonize
        eprintln!("Daemon mode is not supported on Windows.");
        std::process::exit(1);
    }
}

pub fn stop_daemon(pid_file: &str) -> Result<()> {
    let pid_path = std::path::Path::new(pid_file);
    if pid_path.exists() {
        let pid_str = std::fs::read_to_string(pid_file)?;
        let pid = pid_str.trim().parse::<i32>()?;

        info!("Stopping process with PID: {}", pid);

        // 尝试发送 SIGTERM 信号
        let status = Command::new("kill").arg(pid.to_string()).status()?;

        if status.success() {
            info!("Process stopped successfully.");
            let _ = std::fs::remove_file(pid_file);
        } else {
            // 如果 kill 失败，检查进程是否还存在
            let check_status = Command::new("kill")
                // .arg("-9")
                .arg(pid.to_string())
                .status()?;

            if !check_status.success() {
                // 进程已不存在，说明是过期的 PID 文件 (Stale PID)
                info!("Process {} not found. Removing stale PID file.", pid);
                let _ = std::fs::remove_file(pid_file);
            } else {
                // 进程存在但无法停止 (权限不足等)
                eprintln!(
                    "Failed to stop process with PID: {}. Permission denied?",
                    pid
                );
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("PID file not found: {}. Is the server running?", pid_file);
        std::process::exit(1);
    }
    Ok(())
}
