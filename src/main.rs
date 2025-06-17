use cardware_btc::Wallet;
use cardware_btc::array_to_hex;
use cardware_btc::convert_psbt_to_qr;
use std::fs;

#[tokio::main]
async fn main() {
    //Regular Wallet
    //=============================
    /*
    let mut wal = Wallet::new(["zpub6qZke5jbex3WoeqEe8CrBbVozHzsJarGAjh5nKhdaKGNRAWn868QrMbjGLw51dJYEGwBok6iGYx2H3fkZFL9cxvJnKFaaSNzAsMvYJkeQgE".to_string()].to_vec(),
                              "https://btc.cardwarewallet.com".to_string(),
                              "bitcoin".to_string()); */

    //Multi-Sig Walllet
    //=============================
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
      let amount_now = 1000;
      let amount_arr = vec![amount_now];
      println!("fee {:?}",fee);
      println!("amount now {:?}",amount_now);
      let res = wal.send(recipient_arr,amount_arr,fee);
      println!("Res: {:?}",res);
    }

    //Tx History tests
    let tx_res = wal.get_tx_history("m/0/0".to_string()).await;
    println!("tx res: {:?}",tx_res);
}