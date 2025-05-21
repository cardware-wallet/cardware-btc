use cardware_btc::Wallet;
use cardware_btc::array_to_hex;
use cardware_btc::convert_psbt_to_qr;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zpub = "zpub6rTqkZF6uTJgu3Hye8gg2imLMLjfYRfLquBbXP5FwEJPTFS188D6PDr4aDiXjdw2oLVjMjxacjrksENLC6nS1kf6wLGC3M1TuZVRF8HNk1D";
    let esplora_url = "https://btc.cardwarewallet.com";
    
    // Create a wallet instance
    let mut wallet = Wallet::new(vec![zpub.to_string()], esplora_url.to_string(), "bitcoin".to_string());
    
    // Sync the wallet before getting transaction history
    println!("Syncing wallet...");
    let sync_result = wallet.sync().await;
    println!("Sync result: {}", sync_result);
    
    println!("Wallet balance: {} satoshis", wallet.balance());
    
    // Fetch transaction history
    println!("\nFetching transaction history...");
    match wallet.get_tx_history().await {
        Ok(tx_history) => {
            println!("Transaction history: {}", tx_history.as_string().unwrap_or_default());
        },
        Err(e) => println!("Error fetching transaction history: {:?}", e),
    }
    
    Ok(())
}