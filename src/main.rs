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
    
    // Fetch transaction history
    println!("\nFetching transaction history...");
    match wallet.get_tx_history_internal().await {
        Ok(txs) => {
            println!("Found {} transactions:", txs.len());
            
            // Get wallet addresses for transaction analysis
            let wallet_addresses = wallet.get_wallet_addresses();
            
            for tx in txs {
                println!("\nTransaction: {}", tx.txid);
                println!("Block Height: {:?}", tx.status.block_height);
                println!("Confirmed: {}", tx.status.confirmed);
                
                // Display timestamp if available
                if let Some(timestamp) = tx.status.timestamp {
                    println!("Timestamp: {} ({})", timestamp, format_timestamp(timestamp));
                }
                
                // Calculate transaction values
                let total_in = tx.total_input_value();
                let total_out = tx.total_output_value();
                let fee = tx.fee();
                let received = tx.value_received_by_wallet(&wallet_addresses);
                let sent = tx.value_sent_from_wallet(&wallet_addresses);
                
                println!("Total Input Value: {} satoshis", total_in);
                println!("Total Output Value: {} satoshis", total_out);
                println!("Fee: {} satoshis", fee);
                
                if sent > 0 {
                    println!("Sent from wallet: {} satoshis", sent);
                    
                    // Show external recipients
                    println!("Recipients:");
                    for (address, amount) in tx.external_outputs(&wallet_addresses) {
                        println!("  - {} satoshis to {}", amount, address);
                    }
                }
                
                if received > 0 {
                    println!("Received to wallet: {} satoshis", received);
                }
                
                // Net effect on wallet
                let net_value = if sent > 0 {
                    (received as i64) - (sent as i64)
                } else {
                    received as i64
                };
                
                if sent > 0 && received > 0 {
                    println!("Net change: {} satoshis", net_value);
                } else if sent > 0 {
                    println!("Net change: -{} satoshis (including {} fee)", sent + fee, fee);
                } else if received > 0 {
                    println!("Net change: +{} satoshis", received);
                }
                
                println!("Inputs: {}", tx.vin.len());
                println!("Outputs: {}", tx.vout.len());
            }
        },
        Err(e) => println!("Error fetching transaction history: {:?}", e),
    }
    
    Ok(())
}