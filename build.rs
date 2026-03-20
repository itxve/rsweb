// build.rs
use fs_extra::dir::{copy, CopyOptions};
use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    // 1. 准备 Sidecar 资源
    prepare_sidecars()?;

    // 2. 告知 Cargo 监听 web/dist 目录的变化
    // 如果用户的前端构建流程更新了 dist 目录，Cargo 会重新运行 build.rs
    println!("cargo:rerun-if-changed=web/dist");

    Ok(())
}

/// 准备 Sidecar 二进制文件
fn prepare_sidecars() -> io::Result<()> {
    println!("cargo:rerun-if-changed=sidecar/");

    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    let source_dir = Path::new("sidecar/").join(&target);
    let bin_dir = Path::new("bin");

    // 确保 bin 目录存在
    if !bin_dir.exists() {
        fs::create_dir_all(bin_dir)?;
    }

    if source_dir.exists() && source_dir.is_dir() {
        println!("cargo:info=正在准备平台 {} 的 sidecar 资源...", target);
        let options = CopyOptions::new().overwrite(true).content_only(true);
        if let Err(e) = copy(&source_dir, bin_dir, &options) {
            println!("cargo:warning=无法复制 sidecar 文件: {}", e);
        }
    } else {
        println!(
            "cargo:warning=未找到目标平台 {} 的 sidecar 目录: {:?}",
            target, source_dir
        );
    }

    Ok(())
}
