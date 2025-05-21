use cardware_btc::Wallet;
use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};

fn format_timestamp(timestamp: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

fn format_btc(satoshis: u64) -> String {
    format!("{} satoshis ({:.8} BTC)", satoshis, satoshis as f64 / 100_000_000.0)
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
    println!("Wallet balance: {}", format_btc(wallet.balance()));
    
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
                
                println!("Sent: {}", format_btc(tx.sent));
                println!("Received: {}", format_btc(tx.received));
                println!("Fee: {}", format_btc(tx.fee));
                
                // Show external recipients
                if !tx.external_recipients.is_empty() {
                    println!("External Recipients:");
                    for (address, amount) in &tx.external_recipients {
                        println!("  → {} to {}", format_btc(*amount), address);
                    }
                }
                
                // Calculate net effect
                if tx.sent > 0 && tx.received > 0 {
                    let net = (tx.received as i64) - (tx.sent as i64);
                    if net >= 0 {
                        println!("Net change: +{}", format_btc(net as u64));
                    } else {
                        println!("Net change: -{}", format_btc((-net) as u64));
                    }
                } else if tx.sent > 0 {
                    println!("Net change: -{}", format_btc(tx.sent));
                } else if tx.received > 0 {
                    println!("Net change: +{}", format_btc(tx.received));
                }
            }
        },
        Err(e) => println!("Error fetching transaction history: {:?}", e),
    }
    
    Ok(())
}