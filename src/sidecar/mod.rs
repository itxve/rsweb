use anyhow::{anyhow, Result};
use rust_embed::Embed;
use std::process::Stdio;
use std::{fs, path::PathBuf};
use tempfile::NamedTempFile;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

#[derive(Embed)]
#[folder = "bin/"]
struct SideCarAsset;

/// Sidecar 管理器，负责从嵌入资源中提取并运行二进制文件
pub struct Sidecar {
    bin_name: String,
    // 保持对临时文件的引用，确保文件在 Sidecar 存在期间不被删除
    _temp_file: NamedTempFile,
    path: PathBuf,
}

impl Sidecar {
    /// 从嵌入资源中创建一个新的 Sidecar 实例
    pub fn new(bin_name: &str) -> Result<Self> {
        let embedded_name = if cfg!(windows) {
            format!("{}.exe", bin_name)
        } else {
            bin_name.to_string()
        };

        let data = SideCarAsset::get(&embedded_name)
            .ok_or_else(|| anyhow!("Embedded binary '{}' not found", embedded_name))?
            .data;

        // 创建临时文件
        let file = NamedTempFile::new()?;
        fs::write(file.path(), &data)?;

        // 设置可执行权限 (Unix)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(file.path())?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(file.path(), perms)?;
        }

        let path = file.path().to_owned();

        Ok(Sidecar {
            bin_name: bin_name.to_string(),
            _temp_file: file,
            path,
        })
    }

    /// 获取二进制文件的临时路径
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// 准备启动 Sidecar 进程的命令
    pub fn command(&self) -> Command {
        let mut cmd = Command::new(&self.path);
        // 默认配置
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd
    }

    /// 启动 Sidecar 进程并开始记录日志
    pub async fn run_and_log(&self, args: &[&str]) -> Result<()> {
        let mut child = self.spawn(args)?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Failed to capture stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow!("Failed to capture stderr"))?;

        let bin_name = self.bin_name.clone();
        let bin_name_err = bin_name.clone();

        // 异步读取 stdout
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                println!("[{}] {}", bin_name, line);
            }
        });

        // 异步读取 stderr
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                println!("[{}] {}", bin_name_err, line);
            }
        });

        // 等待进程结束
        let status = child.wait().await?;
        if status.success() {
            println!("Sidecar '{}' exited successfully", self.bin_name);
        } else {
            println!("Sidecar '{}' exited with status: {}", self.bin_name, status);
        }

        Ok(())
    }

    /// 启动 Sidecar 进程
    pub fn spawn(&self, args: &[&str]) -> Result<Child> {
        let mut cmd = self.command();
        cmd.args(args);
        Ok(cmd.spawn()?)
    }

    /// 获取二进制名称
    pub fn name(&self) -> &str {
        &self.bin_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sidecar() -> Result<()> {
        // 注意：这里需要 bin/ 目录下确实有文件才能运行成功
        // 示例：Sidecar::new("gic")?
        let sidecar = Sidecar::new("gic")?;
        sidecar.run_and_log(&["--help"]).await?;
        Ok(())
    }
}
