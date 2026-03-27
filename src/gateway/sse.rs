use axum::response::sse::{Event, Sse};
use futures::stream::Stream;
use std::time;
use std::{convert::Infallible, time::Duration};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt as _;

/// 处理 SSE 事件流
pub async fn handle_sse_events() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // 创建一个每 10 秒发送一次心跳的流
    let interval = tokio::time::interval(Duration::from_secs(10));
    let heartbeat = IntervalStream::new(interval).map(|_| {
        let data = serde_json::json!({ "type": "heartbeat","now": time::SystemTime::now() });
        Ok(Event::default().data(data.to_string()))
    });

    // 这里可以根据需要添加更多事件，例如系统状态更新、文件变化通知等
    // 目前仅返回心跳流
    Sse::new(heartbeat).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}
