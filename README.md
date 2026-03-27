# rsweb-template

这是一个轻量、可靠的 Rust Web 基础模板，旨在为您提供一个舒适的起点，集成了实用的守护进程管理与嵌入式静态资源分发功能。

---

## ✨ 核心亮点

- **从容的部署体验**: 通过 GitHub Actions 自动化流程，为您默默打理好各平台的编译与发布工作。
- **静默运行的守护者**: 内置简洁的守护进程管理（`--daemon` / `--stop`），让服务在后台安稳运行，不打扰您的终端。
- **灵活的前端集成**: `web/dist` 下的内容会被打包到最终的二进制文件中，您可以自由挑选喜欢的前端工具，而无需担心后端的限制。
- **清晰的代码职责**: 模块化的结构设计，每个部分都各司其职，方便您根据直觉找到并修改代码。
- **优雅的自我适配**: 所有的路径和名称都与 `Cargo.toml` 保持同步，当您想要给项目起个新名字时，它会体贴地自动更新所有配置。

---

## 🍃 快速开启旅程

我们推荐使用 `cargo-generate` 来开启您的新项目，它会让重命名工作变得自然而顺畅。

```bash
# 1. 准备好工具
cargo install cargo-generate

# 2. 播下种子
cargo generate --git https://github.com/itxve/rsweb-template.git
# 按照您的心意输入项目名称

# 3. 开启旅程
cd <您的项目名>
cargo run
```

---

## 📂 结构概览

```text
.
├── src/                # 后端的核心逻辑，模块清晰
│   ├── gateway/        # Web 网关，包含路由、API、状态管理等
│   │   ├── base.rs     # 基础工具：统一响应格式 (ApiResponse) 和错误处理 (AppError)
│   │   ├── api.rs      # API 业务逻辑实现
│   │   └── ...
├── web/                # 属于前端的自由天地，只需构建至 dist/
├── sidecar/            # 存放辅助程序的小仓库
└── build.rs            # 默默工作的资源打包脚本
```

---

## 🛠️ API 接口说明

- **GET `/api/health`**: 检查服务的健康状态。
- **GET `/api/id`**: 获取当前计数值。
- **GET `/api/id_add`**: 计数值加一。
- **GET `/api/events`**: 获取 SSE 事件流。
- **WS `/ws/chat`**: WebSocket 聊天代理。

---

## 💎 开发指南

### 统一响应 (base.rs)

项目在 `src/gateway/base.rs` 中定义了统一的响应结构 `ApiResponse<T>`，确保所有 API 返回一致的 JSON 格式：

```json
{
  "code": 0,
  "msg": "success",
  "data": { ... }
}
```

- **AppError**: 集中式错误处理，自动将 Rust 错误转换为对应的 HTTP 状态码和 JSON 响应。
- **ToRes**: 便捷的 Trait，支持通过 `.ok()` 快速构造成功响应。

希望这个模板能让您的 Rust 开发过程变得更加轻松和愉悦。
