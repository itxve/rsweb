use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::{debug, error, info};

/// 处理 WebSocket 握手
pub async fn handle_ws_chat(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

/// 实际处理 WebSocket 连接
async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    info!("WebSocket connection established");

    // 循环读取消息并原样返回 (Echo)
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(msg) => {
                if let Message::Text(text) = &msg {
                    debug!("Received message: {}", text);
                }

                // 将接收到的消息原样返回
                if let Err(e) = sender.send(msg).await {
                    error!("Error sending message: {}", e);
                    break;
                }
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    info!("WebSocket connection closed");
}
