use bemp::QuoteManager;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = QuoteManager::check()?;
    let manager = QuoteManager::new(config);
    manager.start().await;
    let some = manager.quotes.read().await;
    println!("Some data: {:?}", some);
    tokio::signal::ctrl_c().await?;


    Ok(())
}
