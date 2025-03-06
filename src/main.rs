use cardware_btc::Wallet;

#[tokio::main]
async fn main() {
    //Regular Wallet
    /*
    let mut wal = Wallet::new(["zpub6qZke5jbex3WoeqEe8CrBbVozHzsJarGAjh5nKhdaKGNRAWn868QrMbjGLw51dJYEGwBok6iGYx2H3fkZFL9cxvJnKFaaSNzAsMvYJkeQgE".to_string()].to_vec(),
                              "https://btc.cardwarewallet.com".to_string(),
                              "bitcoin".to_string()); */

    //Multi-Sig Walllet
    let mut wal = Wallet::new(["2/2".to_string(),
                              "zpub6qhLodRvBBKKmnMHWf3SUgucomzpcR8mRRw9V8sW9sEiLbVDwwN4N5y5tfojPWhKkuxyTtCsuv4W45q9MuxB95iFBVD2mXEyHrkmka1Woxq".to_string(),
                              "zpub6r3zjPVqSa5pc6Qv2fkvghMnggZrdxoZBH26d7L852khEQTGsxQ1ciWmsCKPvCHDafNLri5pycgC8GxZBw2qgjbshJGkfWe89vsNgUW7Qpp".to_string()].to_vec(),
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
    //let fee = wal.estimate_sweep_fee(2);
    let fee = 1000;
    if wal.balance() > fee{
      let amount_now = 2000;
      let amount_arr = vec![amount_now];
      println!("fee {:?}",fee);
      println!("amount now {:?}",amount_now);
      let res = wal.send(recipient_arr,amount_arr,fee);
      println!("Res: {:?}",res);
    }

    let result = wal.broadcast_multisig(["AgAAAAABAVtn+/Z62Ng/+D4ThlNEJwOk6ceVW8XtYo27FbhFrX+iAAAAAAD/////AtAHAAAAAAAAFgAUg3fJmFw+Xphk3Et2NlpZUoP4XyfQBwAAAAAAACIAIFprpate60P957Lu+mdMn+4GojRqTilth+Ju0tRQK+P6A0cwRAIgB5M/8mDBembOzKRh3WQaVcLiDIXzkrQU7CYukgUe6/cCIB+rVr2s6wQBbYME/qleaCHzcMGCgIo3KAsQEEGADTpNASECcikWYSZ3j4ahirRPBY3ZAasVUf7KelhHCdevmCn5MgVHUiECGXDMUG+QxA779XjIsWRTtZfDH1A3SV/KrTvZh6e9CJUhAnIpFmEmd4+GoYq0TwWN2QGrFVH+ynpYRwnXr5gp+TIFUq4AAAAA".to_string(),
                                          "AgAAAAABAVtn+/Z62Ng/+D4ThlNEJwOk6ceVW8XtYo27FbhFrX+iAAAAAAD/////AtAHAAAAAAAAFgAUg3fJmFw+Xphk3Et2NlpZUoP4XyfQBwAAAAAAACIAIFprpate60P957Lu+mdMn+4GojRqTilth+Ju0tRQK+P6A0gwRQIhAJu+voRxIcA2WeaBDlPFvgtrhyGRosOUeWlBktyKD8qcAiBsa0o7Mrevy3Vg56dijmo30rL6pd9GIZ+P1w4pCEa9QgEhAhlwzFBvkMQO+/V4yLFkU7WXwx9QN0lfyq072YenvQiVR1IhAhlwzFBvkMQO+/V4yLFkU7WXwx9QN0lfyq072YenvQiVIQJyKRZhJnePhqGKtE8FjdkBqxVR/sp6WEcJ16+YKfkyBVKuAAAAAA==".to_string()].to_vec()).await;
    println!("b result: {:?}",&result);
}