use cardware_btc::Wallet;

#[tokio::main]
async fn main() {
    let mut wal = Wallet::new("vpub5ZNhc5KKM6hACK6QDuo6UG1749XUeXf9Gbu8rcZQnNDeMJwUPrwzEVKsF7X7EzZe5yqwymfMA1tGJ9qAmjdmGHSkRW7SruCEDz9mgEkwWvN".to_string(),
                              "https://blockstream.info/testnet/api".to_string(),
                              "testnet".to_string());
    
    println!("wallet addr: {:?}",wal.address());
    println!("wallet addr: {:?}",wal.new_address("m/0/0"));
    println!("wallet addr: {:?}",wal.new_address("m/0/1"));
    let sync_res = wal.sync().await;
    println!("sync res {:?}",sync_res);
    println!("wallet btc: {:?}",wal.balance());
    let recipient_arr = vec![
        "tb1qvdl9rvg3m5ghfnppw2728rd92059pfqe0a8jjv".to_string(),
    ];
    let amount_arr = vec![4500];
    let fee = wal.estimate_fee(recipient_arr.clone(),amount_arr.clone(),2);
    println!("fee {:?}",fee);
    let res = wal.send(recipient_arr,amount_arr,550);
    println!("Res: {:?}",res);

}