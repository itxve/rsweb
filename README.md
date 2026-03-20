# RsWeb Template

这是一个高性能、跨平台的 Rust Web 服务模板，集成了守护进程管理、Sidecar 模式、以及嵌入式前端。

## 🌟 特性

- **跨平台支持**: 针对 Linux、Windows 和 macOS 进行了优化。
- **守护进程管理**:
  - `-d, --daemon`: 一键后台运行。
  - `-s, --stop`: 智能停止后台进程，自动清理过期 PID 文件。
  - 自动管理 PID 文件和日志路径 (`/tmp/com.rsweb/`)。
- **Sidecar 模式**:
  - 支持从二进制资源中动态提取并运行辅助程序。
  - 异步日志捕获，Sidecar 日志自动集成到主程序。
  - 生命周期自动管理，程序退出时自动清理临时 Sidecar 文件。
- **嵌入式前端**:
  - 集成 React + Vite 开发环境。
  - `build.rs` 自动化构建：修改前端代码后，`cargo build` 会自动完成 `npm install` 和 `npm build`。
  - 使用 `rust-embed` 将前端产物打包进单一二进制文件。
- **高性能网关**: 基于 Axum + Tokio，支持高性能并发处理。
- **模块化结构**: 职责分明，易于扩展。

## 🚀 快速开始

### 1. 准备工作

确保已安装：

- Rust (最新稳定版)
- Node.js & npm (用于前端构建)

### 2. 开发运行

```bash
# 启动 Web 服务 (默认端口 41218)
cargo run

# 开启详细日志
cargo run -- -v
```

### 3. 守护进程模式

```bash
# 后台启动
./target/debug/rsweb --daemon

# 查看后台日志
tail -f /tmp/com.rsweb/rsweb.out

# 停止后台进程
./target/debug/rsweb --stop
```

## 📂 项目结构

```text
.
├── src/
│   ├── daemon/      # 守护进程逻辑 (PID管理, fork等)
│   ├── gateway/     # Axum 路由与静态文件分发
│   ├── sidecar/     # Sidecar 二进制管理逻辑
│   ├── utils/       # 日志初始化等工具
│   └── main.rs      # 入口逻辑与 CLI 定义
├── web/             # 前端 React 项目
├── sidecar/         # 存放不同平台的 sidecar 二进制文件
├── bin/             # (自动生成) 存放编译时提取的 sidecar
└── build.rs         # 自动化构建脚本 (前端编译 & 资源准备)
```

## 🛠️ 自定义配置

- **跳过前端构建**: 如果只想调试后端，可以设置环境变量：
  `SKIP_WEB_BUILD=1 cargo build`
- **修改服务名称**: 在 `src/service.rs` (如果启用) 或 `src/daemon.rs` 中修改相关常量。

## 📦 编译发布

```bash
# 优化体积的发布编译
cargo build --release
```

产物将位于 `target/release/rsweb`，是一个包含了所有前端资源的独立二进制文件。
