use tracing::{info, Level};

pub fn init_tracing(log_level: &str) {
    tracing_subscriber::fmt()
        .with_max_level(level_from_str(log_level))
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("✅ Tracing initialized with level: {}", log_level);
}

fn level_from_str(level: &str) -> Level {
    match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    }
}
