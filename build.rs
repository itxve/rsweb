// build.rs
use fs_extra::dir::{copy, CopyOptions};
use std::{env, io, path::Path};

// 用于实现 sidecar

fn main() -> io::Result<()> {
    // 文件变更触发build
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=sidecar/");
    println!("cargo:info=。。。。。。");

    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    let options = CopyOptions::new()
        .overwrite(true)
        .content_only(true)
        .depth(1);

    let source_target_bin = Path::new("sidecar/").join(target);

    copy(source_target_bin, "bin", &options).unwrap();

    Ok(())
}
