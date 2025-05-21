use cardware_btc::get_tx_history_from_zpub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zpub = "zpub6rTqkZF6uTJgu3Hye8gg2imLMLjfYRfLquBbXP5FwEJPTFS188D6PDr4aDiXjdw2oLVjMjxacjrksENLC6nS1kf6wLGC3M1TuZVRF8HNk1D";
    let esplora_url = "https://btc.cardwarewallet.com";
    
    println!("Fetching transaction history for zpub...");
    match get_tx_history_from_zpub(zpub, esplora_url).await {
        Ok(txs) => {
            println!("\nFound {} transactions:", txs.len());
            for tx in txs {
                println!("\nTransaction: {}", tx.txid);
                println!("Block Height: {:?}", tx.status.block_height);
                println!("Confirmed: {}", tx.status.confirmed);
                println!("Inputs: {}", tx.vin.len());
                println!("Outputs: {}", tx.vout.len());
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}