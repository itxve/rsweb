use anyhow::Result;

mod gateway;

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> Result<()> {
    println!("🦀 ZeroClaw Status");

    gateway::run_gateway("127.0.0.1", 9910).await
}
