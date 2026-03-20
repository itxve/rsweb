# {{project_name}}: 高性能 Rust Web 服务模板

[![使用此模板](https://img.shields.io/badge/-%E4%BD%BF%E7%94%A8%E6%AD%A4%E6%A8%A1%E6%9D%BF-brightgreen?style=for-the-badge&logo=github)](https://github.com/new?template_name={{project_name}}&template_owner=你的用户名)

这是一个生产级的 Rust Web 服务模板，旨在提供开箱即用的高性能后端、自动化构建流程和优雅的跨平台部署方案。

---

## 🚀 快速开始 (使用 `cargo-generate`)

这是使用此模板的最佳方式，它会自动为你完成项目创建和重命名。

```bash
# 1. 安装 cargo-generate
cargo install cargo-generate

# 2. 使用模板生成你的新项目
cargo generate --git https://github.com/你的用户名/{{project_name}}
# 按照提示输入你的项目名称，例如：my-awesome-app

# 3. 进入项目并开始开发
cd my-awesome-app
cargo run
```

---

## 🌟 核心特性

- **与前端技术栈无关**: `build.rs` 只负责将 `web/dist` 目录下的任何静态资源嵌入到最终的二进制文件中。你可以自由使用 React, Vue, Svelte, 或纯 HTML。
- **全自动 CI/CD**: 使用 GitHub Actions，在推送 `v*` 标签时自动构建并发布适用于 Linux, Windows, macOS (Intel & ARM) 的二进制文件。
- **工业级守护进程**:
  - 一键后台运行 (`--daemon`)，智能停止 (`--stop`)。
  - 自动处理 PID 文件存活检测和过期清理。
- **Sidecar 模式**:
  - 轻松集成并管理外部二进制程序。
  - 异步捕获 Sidecar 的日志流，并自动重定向到主程序日志。
- **动态配置**:
  - **零硬编码**: 项目名称、PID 路径、日志路径等均基于 `Cargo.toml` 动态生成。
- **高性能 Web 服务**: 基于 Axum 和 Tokio，提供异步、高并发的网络处理能力。

---

## 🛠️ 开发流程

1.  **前端开发**: 在 `web/` 目录下进行开发。完成后，确保你的构建产物输出到 `web/dist` 目录。

    ```bash
    cd web
    npm run build # 或其他构建命令
    ```

2.  **后端开发**:
    - **运行开发服务器**: `cargo run`
    - **后台运行**: `./target/debug/{{project_name}} --daemon`
    - **停止后台进程**: `./target/debug/{{project_name}} --stop`
    - **查看后台日志**: `tail -f /tmp/com.{{project_name}}/{{project_name}}.out`
    - **编译发布版本**: `cargo build --release`

---

## 📂 项目结构

```text
.
├── .github/workflows/  # CI/CD 自动化配置
├── src/                # Rust 后端代码
├── web/                # 前端项目 (技术栈任选)
│   └── dist/           # 前端构建产物目录 (唯一需要关注的)
├── sidecar/            # 存放不同平台的 sidecar 二进制文件
├── bin/                # (自动生成) 存放编译时提取的 sidecar
├── build.rs            # 自动化构建脚本 (只负责嵌入资源)
└── cargo-generate.toml # cargo-generate 配置文件
```
