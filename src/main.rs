use cardware_btc::Wallet;
use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};

fn format_timestamp(timestamp: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zpub = "zpub6qWnLWskXjBhPC3xaBd1ZBQUP1qmjuoH2H67jfZqetTKU5LjcJkicLdoa1iDSfgrd2Bw2a1gdCirKvUQ6kxffe8yNNENPCoDS68MqfBcXyb";
    let esplora_url = "https://btc.cardwarewallet.com";
    
    // Create a wallet instance
    let mut wallet = Wallet::new(vec![zpub.to_string()], esplora_url.to_string(), "bitcoin".to_string());
    
    // Sync the wallet before getting transaction history
    println!("Syncing wallet...");
    let sync_result = wallet.sync().await;
    println!("Sync result: {}", sync_result);
    println!("Wallet balance: {} satoshis", wallet.balance());
    
    // Fetch transaction history with simpler implementation
    println!("\nFetching transaction history...");
    match wallet.get_tx_history_internal().await {
        Ok(txs) => {
            println!("Found {} transactions:", txs.len());
            
            for tx in txs {
                println!("\nTransaction: {}", tx.txid);
                println!("Confirmed: {}", tx.confirmed);
                
                if let Some(height) = tx.block_height {
                    println!("Block Height: {}", height);
                }
                
                if let Some(timestamp) = tx.timestamp {
                    println!("Time: {}", format_timestamp(timestamp));
                }
                
                println!("Sent: {} satoshis", tx.sent);
                println!("Received: {} satoshis", tx.received);
                println!("Fee: {} satoshis", tx.fee);
                
                // Calculate net effect
                if tx.sent > 0 && tx.received > 0 {
                    let net = (tx.received as i64) - (tx.sent as i64);
                    println!("Net change: {} satoshis", net);
                } else if tx.sent > 0 {
                    println!("Net change: -{} satoshis", tx.sent + tx.fee);
                } else if tx.received > 0 {
                    println!("Net change: +{} satoshis", tx.received);
                }
            }
        },
        Err(e) => println!("Error fetching transaction history: {:?}", e),
    }
    
    Ok(())
}