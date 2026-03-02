mod gateway;
use axum::{
    Router,
    extract::Path,
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use rust_embed::Embed;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 嵌入静态资源目录
#[derive(Embed)]
#[folder = "web/dist/"]
struct StaticAssets;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Axum static file server...");

    // 构建路由
    let app = Router::new()
        // 主页
        .route("/", get(serve_home))
        // 静态文件路由
        .route("/assets/*path", get(serve_static_file))
        // 通配符路由，捕获所有其他路径（支持 SPA 路由）
        .route("/*path", get(serve_static_file))
        .layer(TraceLayer::new_for_http());

    // 设置地址和端口
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on http://{}", addr);

    // 启动服务器
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 服务首页
async fn serve_home() -> impl IntoResponse {
    serve_file("index.html").await
}

// 服务静态文件
async fn serve_static_file(Path(path): Path<String>) -> impl IntoResponse {
    serve_file(&path).await
}

// 主要的文件服务逻辑
async fn serve_file(path: &str) -> Response {
    // 如果路径为空，返回首页
    let path = if path.is_empty() { "index.html" } else { path };

    // 尝试获取文件
    match StaticAssets::get(path) {
        Some(content) => {
            // 根据文件扩展名设置 Content-Type
            let mime_type = match get_file_extension(path) {
                "html" => mime::TEXT_HTML_UTF_8,
                "css" => mime::TEXT_CSS_UTF_8,
                "js" => mime::APPLICATION_JAVASCRIPT_UTF_8,
                "json" => mime::APPLICATION_JSON,
                "png" => mime::IMAGE_PNG,
                "jpg" | "jpeg" => mime::IMAGE_JPEG,
                "gif" => mime::IMAGE_GIF,
                "svg" => mime::IMAGE_SVG,
                "ico" => mime::IMAGE_PNG, // ico 通常用 PNG
                "woff" => mime::FONT_WOFF,
                "woff2" => mime::FONT_WOFF2,
                "ttf" => mime::FONT_TTF,
                "otf" => mime::FONT_OTF,
                _ => mime::APPLICATION_OCTET_STREAM,
            };

            // 构建响应
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .header(header::CACHE_CONTROL, "public, max-age=3600")
                .body(axum::body::boxed(axum::body::Full::from(content.data)))
                .unwrap()
        }
        None => {
            // 文件不存在，尝试返回 404.html
            match StaticAssets::get("404.html") {
                Some(content) => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header(header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8.as_ref())
                    .body(axum::body::boxed(axum::body::Full::from(content.data)))
                    .unwrap(),
                None => {
                    // 没有 404.html，返回简单文本
                    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
                }
            }
        }
    }
}

// 获取文件扩展名
fn get_file_extension(path: &str) -> &str {
    path.rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase()
        .as_str()
}
