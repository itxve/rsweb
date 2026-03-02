use anyhow::Result;

mod gateway;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦀 Status");

    gateway::run_gateway("127.0.0.1", 41111).await
}
