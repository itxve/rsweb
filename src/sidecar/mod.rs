use std::{
    fs,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use rust_embed::Embed;
use tempfile::NamedTempFile;

#[derive(Embed)]
#[folder = "bin/"]
pub struct SideCarAsset;

pub struct Sidecar {
    _temp_file: NamedTempFile,
    path: std::path::PathBuf,
}

impl Sidecar {
    pub fn new(bin_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 根据平台选择嵌入的文件名
        let embedded_name = if cfg!(windows) {
            format!("{}.exe", bin_name)
        } else {
            bin_name.to_string()
        };

        let data = SideCarAsset::get(&embedded_name)
            .ok_or(format!("{} not found", embedded_name))?
            .data;

        // 创建临时文件
        let mut file = NamedTempFile::new_in("")?;

        // 向临时文件写入数据
        fs::write(file.path(), &data)?;

        // 设置可执行权限
        let metadata = fs::metadata(file.path())?;
        let mut perms = metadata.permissions();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            perms.set_mode(0o755);
            fs::set_permissions(file.path(), perms)?;
        }

        let path = file.path().to_owned();

        Ok(Sidecar {
            _temp_file: file,
            path,
        })
    }

    fn path(&self) -> &std::path::Path {
        &self.path
    }

    // 如果你希望提供启动进程的便捷方法，但让调用方自己配置Command
    fn create_command(&self) -> Command {
        let mut cmd = Command::new(self.path());
        cmd
    }
}

// 调用方使用示例
#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    // 创建sidecar
    let sidecar = Sidecar::new("rsweb")?;

    // 获取路径
    let path = sidecar.path();
    println!("可执行文件路径: {:?}", path);

    // 创建命令
    let mut cmd = Command::new(path);

    // 调用方可以自己配置命令参数
    cmd.arg("--help")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // 启动进程
    let mut child = cmd.spawn()?;

    // 读取输出
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("输出: {}", line),
                Err(e) => eprintln!("读取错误: {}", e),
            }
        }
    }

    // 等待进程结束
    let status = child.wait()?;
    println!("进程退出状态: {}", status);

    // Sidecar在离开作用域时会自动清理临时文件
    Ok(())
}
