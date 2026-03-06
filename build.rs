// build.rs
use fs_extra::dir::{copy, CopyOptions};
use std::{env, fs, io, path::Path};

// 用于实现 sidecar

fn main() -> io::Result<()> {
    // 文件变更触发build
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=sidecar/");
    println!("cargo:info=正在处理 sidecar 资源...");

    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    let options = CopyOptions::new().overwrite(true).content_only(true);

    let source_target_bin = Path::new("sidecar/").join(target.clone());

    // 检查源目录是否存在
    if !source_target_bin.exists() || !source_target_bin.is_dir() {
        println!(
            "cargo:warning=目标平台 {} 的 sidecar 目录不存在: {:?}",
            target.clone(),
            source_target_bin
        );
        println!("cargo:warning=将不复制任何 sidecar 文件");
        return Ok(());
    }

    // 确保目标目录存在
    let target_dir = Path::new("bin");
    if !target_dir.exists() {
        fs::create_dir(target_dir)?;
    }

    copy(source_target_bin, "bin", &options).unwrap();

    Ok(())
}
