# rsaxum-web-template

这是一个轻量、可靠的 Rust Web 基础模板，旨在为您提供一个舒适的起点，集成了实用的守护进程管理与嵌入式静态资源分发功能。

---

## ✨ 核心亮点

- **统一的 API 返回规范**: 所有接口围绕 `code / msg / data` 结构组织，便于前后端约定统一的成功与失败语义。
- **清晰的网关分层**: `gateway` 模块按路由入口、基础能力、中间件、状态、用户、SSE、WebSocket 等职责拆分，便于持续演进。
- **内置认证与日志中间件**: 基于 Tower/Axum 中间件机制实现权限校验与请求日志，保证业务 Handler 尽量保持纯粹。
- **静态资源内嵌分发**: `web/dist` 产物可直接嵌入二进制，通过 `/_app/*` 与 SPA fallback 提供前端静态资源。
- **守护进程支持**: 提供 `--daemon` / `--stop` 能力，适合将服务以后台进程方式运行。
- **Sidecar 运行能力**: 内置 sidecar 提取与执行逻辑，方便后续扩展辅助二进制工具。

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

## 📂 当前项目结构

```text
.
├── src/
│   ├── main.rs                 # 程序入口，负责 CLI 解析、日志初始化、运行网关
│   ├── daemon.rs               # 守护进程启动与停止逻辑
│   ├── utils.rs                # tracing 等通用初始化工具
│   ├── sidecar/
│   │   └── mod.rs              # Sidecar 二进制提取、启动与日志转发
│   └── gateway/
│       ├── mod.rs              # 网关总装配：路由、状态、中间件、超时、体积限制、fallback
│       ├── api.rs              # 业务接口 Handler
│       ├── sse.rs              # SSE 心跳事件流
│       ├── ws.rs               # WebSocket Echo 示例
│       ├── state.rs            # 全局状态，目前为 IndexState
│       ├── user/
│       │   └── mod.rs          # 登录接口、用户提取器、Token 校验器
│       └── base/
│           ├── mod.rs          # 统一响应、错误类型、输入提取器、返回包装
│           ├── static_files.rs # 嵌入式静态资源分发与 SPA fallback
│           └── middleware/
│               ├── mod.rs      # 中间件模块出口
│               ├── auth.rs     # 认证中间件
│               └── log.rs      # 日志中间件
├── sidecar/
│   └── README.md               # Sidecar 相关说明
├── web/
│   └── README.md               # 前端目录说明
├── build.rs                    # 构建期资源处理
└── Cargo.toml                  # Rust 依赖与构建配置
```

---

## 🛠️ API 接口说明

- **公开接口**
- **GET `/api/health`**: 健康检查。
- **GET `/api/events`**: SSE 心跳流。
- **POST `/api/user/login`**: 用户登录，返回测试 token。
- **受保护接口**
- **GET `/api/id`**: 获取当前计数值。
- **GET `/api/id_add`**: 计数值加一。
- **GET `/api/user`**: 读取当前用户信息，依赖 `User` 提取器。
- **POST `/api/test/json`**: 测试统一 JSON 提取器。
- **WS `/ws/chat`**: WebSocket Echo 示例，使用独立路由组并套用认证中间件。

---

## 💎 开发指南

### 统一响应

项目在 `src/gateway/base/mod.rs` 中定义了统一的响应结构 `ApiResponse<T>`，确保所有 API 返回一致的 JSON 格式：

```json
{
  "code": 0,
  "msg": "success",
  "data": { ... }
}
```

- **Reply / ApiResult**: Handler 成功返回值包装器，负责自动补齐成功响应格式。
- **AppError**: 集中式错误处理，负责把业务错误映射为统一 JSON 响应。
- **AppJson**: 自定义 JSON 提取器，保证请求体解析失败时也能走统一错误返回。
- **ToApiResult**: 便捷 Trait，支持通过 `.ok()` 与 `.with_code()` 快速构造成功响应。

### 路由组织

- `src/gateway/mod.rs` 使用 `nest("/api", ...)` 区分公开接口与受保护接口。
- 受保护的 API 与 WebSocket 路由会挂载认证中间件。
- 静态文件通过 `/_app/*` 提供，其他页面请求会回落到 SPA 入口。

希望这个模板能让您的 Rust 开发过程变得更加轻松和愉悦。
