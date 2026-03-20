// build.rs
use fs_extra::dir::{copy, CopyOptions};
use std::process::Command;
use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    // 1. 处理 Sidecar 资源
    prepare_sidecars()?;

    // 2. 自动化前端构建 (仅在 web 目录存在且未显式跳过时执行)
    if should_build_frontend() {
        build_frontend()?;
    }

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

/// 判断是否应该执行前端构建
fn should_build_frontend() -> bool {
    let web_dir = Path::new("web");

    // 基础条件：web 目录必须存在
    if !web_dir.exists() {
        return false;
    }

    // 环境变量控制：可以通过设置 SKIP_WEB_BUILD=1 跳过构建
    if env::var("SKIP_WEB_BUILD").is_ok() {
        println!("cargo:info=跳过前端构建 (SKIP_WEB_BUILD 已设置)");
        return false;
    }

    // 监听变更：如果 web 源码有变，cargo 应该重新运行 build.rs
    println!("cargo:rerun-if-changed=web/src");
    println!("cargo:rerun-if-changed=web/public");
    println!("cargo:rerun-if-changed=web/package.json");
    println!("cargo:rerun-if-changed=web/vite.config.ts");

    true
}

/// 执行前端构建逻辑
fn build_frontend() -> io::Result<()> {
    let web_dir = Path::new("web");

    println!("cargo:info=正在检查前端构建环境...");

    // 1. 检查并安装依赖 (如果 node_modules 不存在)
    if !web_dir.join("node_modules").exists() {
        println!("cargo:info=正在安装前端依赖...");
        let npm_install = if cfg!(windows) { "npm.cmd" } else { "npm" };
        let status = Command::new(npm_install)
            .args(&["install"])
            .current_dir(web_dir)
            .status()?;

        if !status.success() {
            println!("cargo:warning=前端依赖安装失败，尝试跳过构建...");
            return Ok(());
        }
    }

    // 2. 执行构建命令
    println!("cargo:info=正在执行前端构建 (npm run build)...");
    let npm_build = if cfg!(windows) { "npm.cmd" } else { "npm" };
    let status = Command::new(npm_build)
        .args(&["run", "build"])
        .current_dir(web_dir)
        .status()?;

    if !status.success() {
        println!("cargo:warning=前端构建失败，请手动检查 web 目录。");
    } else {
        println!("cargo:info=前端构建完成。");
    }

    Ok(())
}
