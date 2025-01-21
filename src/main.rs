use cardware_btc::Wallet;

#[tokio::main]
async fn main() {
    let mut wal = Wallet::new("zpub6qZke5jbex3WoeqEe8CrBbVozHzsJarGAjh5nKhdaKGNRAWn868QrMbjGLw51dJYEGwBok6iGYx2H3fkZFL9cxvJnKFaaSNzAsMvYJkeQgE".to_string(),
                              "https://btc.cardwarewallet.com".to_string(),
                              "bitcoin".to_string());
    
    println!("wallet addr: {:?}",wal.address());
    println!("wallet addr: {:?}",wal.new_address("m/0/0"));
    println!("wallet addr: {:?}",wal.new_address("m/0/1"));
    let sync_res = wal.sync().await;
    println!("sync res {:?}",sync_res);
    println!("wallet btc: {:?}",wal.balance());
    let recipient_arr = vec![
        "bc1qsdmunxzu8e0fsexufdmrvkje22plshe8dye509".to_string(),
    ];
    let fee = wal.estimate_sweep_fee(2);
    let amount_now = wal.balance() - fee;
    let amount_arr = vec![amount_now];
    println!("fee {:?}",fee);
    println!("amount now {:?}",amount_now);
    let res = wal.send(recipient_arr,amount_arr,fee);
    println!("Res: {:?}",res);

}