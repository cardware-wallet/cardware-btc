use wasm_bindgen::prelude::*;
use reqwest::Client;
use std::collections::HashMap;
use std::str::FromStr;
use hex::FromHex;
use serde::{Deserialize,Serialize};
use bitcoin::{Address,Txid,Transaction,TxIn,TxOut,Sequence,Witness,Amount,ScriptBuf,Network};
use bitcoin::address::NetworkChecked;
use bitcoin::blockdata::transaction::OutPoint;
use bitcoin::hashes::Hash;
use bitcoin::absolute::LockTime;
use bitcoin::consensus::Encodable;
use bitcoin::psbt::Psbt;
use std::ffi::c_char;
use std::mem::transmute;
use std::ffi::CStr;
use std::str;
use std::ffi::CString;
use base64::{encode as b64_encode, decode as b64_decode};
use bitcoin::consensus::encode;
use bitcoin::consensus::encode::deserialize;
use bitcoin::bip32::{Xpub, DerivationPath, ChildNumber,Fingerprint};
use bitcoin::hashes::hash160;
use bitcoin::secp256k1::PublicKey;
use hex::{encode, decode};

//Main Wallet Object
#[wasm_bindgen]
pub struct Wallet{
    esplora_url : String,
    fingerprint : String,
    xpub : String,
    network : String,
    btc : u64,
    unconfirmed : u64,
    utxos : Option<Vec<Utxo>>,
    trusted_pending : Option<Vec<String>>,
    fee_estimates : Option<HashMap<String,f64>>,
}

#[wasm_bindgen]
impl Wallet{
    #[wasm_bindgen(constructor)]
    pub fn new(xpub :String, esplora_url : String, fingerprint : String, network : String) -> Wallet {
        Wallet { 
            esplora_url : esplora_url,
            fingerprint : fingerprint,
            xpub : xpub,
            network : network,
            btc : 0,
            unconfirmed : 0,
            utxos : None,
            trusted_pending : None,
            fee_estimates : None,
        }
    }
    pub async fn sync(&mut self){
        self.sync_to_depth("m/0/0".to_string()).await;
    }
    pub async fn sync_to_depth(&mut self, max_depth : String) -> String{
        let fee_sync = match self.fetch_fee_estimates().await{
            Ok(dict) => dict,
            Err(_) => return "Error: Fee sync error.".to_string(),
        };
        self.fee_estimates = Some(fee_sync);
        let derivations = generate_variations(&max_depth);
        let mut utxos = Vec::new();
        let mut btc_bal = 0;
        let mut unconf = 0;
        for derivation in derivations{
            let client = reqwest::Client::new();
            let url_str = format!("{}/address/{}/utxo",&self.esplora_url,&self.new_address(&derivation));
            let response = match client
                .get(url_str)
                .send()
                .await{
                    Ok(response) => response,
                    Err(_) => return "Error: Failed to connect to full node (Esplora).".to_string(),
                };
            if response.status().is_success() {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<Vec<EsploraUtxos>>(&body){
                    Ok(eutxos) => {
                        for etxo in eutxos{
                            let utxo = Utxo{
                                utxo : format!("{}:{}",etxo.txid,etxo.vout),
                                btc : etxo.value,
                                txid : etxo.txid.clone(),
                                confirmed : etxo.status.confirmed,
                                derivation_path: derivation.clone(),
                            };
                            utxos.push(utxo);
                            let mut tx_vec = self.get_trusted_pending();
                            if etxo.status.confirmed {
                                btc_bal += etxo.value;
                                let mut index =0;
                                for tp in &tx_vec{
                                    if tp.clone() == etxo.txid{
                                        tx_vec.remove(index);
                                        break;
                                    }
                                    index += 1;
                                }
                                self.set_trusted_pending(tx_vec);
                            }else{
                                let mut trusted_utxo = false;
                                for tp in tx_vec {
                                    if etxo.txid == tp.clone(){
                                        trusted_utxo = true;
                                        break;
                                    }
                                }
                                if trusted_utxo{
                                    btc_bal += etxo.value;
                                }else{
                                    unconf += etxo.value;
                                }
                            }
                        }
                    }
                    Err(_) => return "Error: Failed to deserialize transaction.".to_string(),
                };
            }else{
                return "Error: Failed to connect to full node (Esplora).".to_string(); 
            }
        }
        self.btc = btc_bal;
        self.unconfirmed = unconf;
        println!("{:?}",utxos.clone() );
        self.utxos = Some(utxos);
        return "Sync successful.".to_string();
    }
    pub async fn broadcast(&mut self, transaction : String) -> String{
        let client = Client::new();
        let tx_hex_string = match base64_to_hex(&transaction){
            Ok(tx_hex_string) => tx_hex_string,
            Err(_) => return "Error: Failed to parse base64 transaction.".to_string(),
        };
        let tx_bytes = match hex::decode(tx_hex_string.clone()){
            Ok(tx_bytes) => tx_bytes,
            Err(_) => return "Error: Decoding failed.".to_string(),
        };
        let txid_str = match deserialize::<bitcoin::Transaction>(&tx_bytes){
            Ok(tx) =>tx.txid().to_string(),
            Err(_) => return "Error: Invalid transaction.".to_string(),
        };
        match client
            .post(format!("{}/tx",&self.esplora_url)) //.header(header::CONTENT_TYPE, "application/json")
            .body(tx_hex_string)
            .send()
            .await{
                Ok(_) => {
                        //Add to trusted pending
                        let mut tp = self.get_trusted_pending();
                        tp.push(txid_str.clone());
                        self.set_trusted_pending(tp);
                        return txid_str;
                    }
                Err(_) => return "Error: Failed to broadcast transaction.".to_string(),
            }
    }
    pub fn send(&self, recipient_addrs : Vec<String>, amounts : Vec<u64>, fee : u64) -> Vec<String>{
        let mut txin_vec = Vec::new();
        let mut txout_vec = Vec::new();
        let mut segwit_ed : Vec<u8> = Vec::new();
        let network = self.get_network();
        let address_str = &self.address();
        if amounts.len() != recipient_addrs.len(){
            return vec!["Error: Recipients and amounts arrays must be the same length.".to_string()];
        }
        let my_address : Address<NetworkChecked> = match Address::from_str(address_str){
            Ok(rec) =>  {
                match rec.require_network(network){
                    Ok(checked) => checked,
                    Err(_) => return vec!["Error".to_string()],
                }
            }
            Err(_) => return vec!["Error: Failed to parse network.".to_string()],
        };
        let mut total_amount : u64 = 0;
        for amount in &amounts{
            total_amount += amount;
        }
        if total_amount + fee > self.btc {
            return vec!["Error: Insufficient funds.".to_string()];
        }
        let mut total_spend : u64 = 0;
        let my_utxos = match &self.utxos{
            Some(utxos)=>utxos,
            None=> return vec!["Error: No UTXOs to spend.".to_string()],
        };
        for utxo in my_utxos{
            let outpoint = convert_to_outpoint(&utxo.utxo);
            total_spend += utxo.btc;
            let txin = TxIn{
                previous_output : outpoint,
                script_sig : ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            };
            txin_vec.push(txin);
            match extract_u16s(&utxo.derivation_path) {
                Ok((first, second)) => append_integers_as_bytes(&mut segwit_ed,first,second,utxo.btc),
                Err(e) => return vec!["Error: Derivation path error.".to_string()],
            }
            if total_spend > total_amount + fee{
                break;
            }
        }
        let mut amt_index =0;
        for recipient_addr in recipient_addrs{
            let recipient : Address<NetworkChecked> = match Address::from_str(&recipient_addr){
                Ok(rec) => {
                    match rec.require_network(network){
                        Ok(checked) => checked,
                        Err(_) => return vec!["Error: Invalid recipient address.".to_string()],
                    }
                }
                Err(_) => return vec!["Error: Invalid recipient address.".to_string()],
            };
            let output  =  TxOut{
                value : Amount::from_sat(amounts[amt_index]),
                script_pubkey: recipient.script_pubkey(),
            };
            txout_vec.push(output);
            amt_index += 1;
        }
        

        let change_amt = total_spend - (total_amount+fee);
        let change = TxOut{
            value : Amount::from_sat(change_amt),
            script_pubkey: my_address.script_pubkey(),
        };
        txout_vec.push(change);


        let locktime = LockTime::from_height(0).expect("Zero always valid.");
        let unsigned_tx = Transaction{
            version: bitcoin::transaction::Version(2),
            lock_time : locktime.into(),
            input : txin_vec.clone(),
            output : txout_vec.clone(),
        };
        let mut serialized_tx = Vec::new();
        let _ = unsigned_tx.consensus_encode(&mut serialized_tx);

        let final_str = base64::encode(&serialized_tx) + ":"+&base64::encode(&segwit_ed);
        return chunk_and_label(&final_str,40);
    }
    pub fn estimate_fee(&self,recipient_addrs : Vec<String>, amounts : Vec<u64>, number_of_blocks : i32) -> u64{
        let mut txin_vec = Vec::new();
        let mut txout_vec = Vec::new();
        let network = self.get_network();
        let address_str = &self.address();
        let fee = 1000;
        if amounts.len() != recipient_addrs.len(){
            return 0; //Array length error
        }
        let my_address : Address<NetworkChecked> = match Address::from_str(address_str){
            Ok(rec) =>  {
                match rec.require_network(network){
                    Ok(checked) => checked,
                    Err(_) => return 1,
                }
            }
            Err(_) => return 1,
        };
        let mut total_amount : u64 = 0;
        for amount in &amounts{
            total_amount += amount;
        }
        if total_amount + fee > self.btc {
            return 4;//Insuffient Funds
        }
        let mut total_spend : u64 = 0;
        let my_utxos = match &self.utxos{
            Some(utxos)=>utxos,
            None=> return 5, //No Utxos
        };
        for utxo in my_utxos{
            let outpoint = convert_to_outpoint(&utxo.utxo);
            total_spend += utxo.btc;
            let txin = TxIn{
                previous_output : outpoint,
                script_sig : ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            };
            txin_vec.push(txin);
            if total_spend > total_amount + fee{
                break;
            }
        }
        let mut amt_index =0;
        for recipient_addr in recipient_addrs{
            let recipient : Address<NetworkChecked> = match Address::from_str(&recipient_addr){
                Ok(rec) => {
                    match rec.require_network(network){
                        Ok(checked) => checked,
                        Err(_) => return 1,//Failed to parse network
                    }
                }
                Err(_) => return 2,//Invalid recipient address
            };
            let output  =  TxOut{
                value : Amount::from_sat(amounts[amt_index]),
                script_pubkey: recipient.script_pubkey(),
            };
            txout_vec.push(output);
            amt_index += 1;
        }
        

        let change_amt = total_spend - (total_amount+fee);
        let change = TxOut{
            value : Amount::from_sat(change_amt),
            script_pubkey: my_address.script_pubkey(),
        };
        txout_vec.push(change);


        let locktime = LockTime::from_height(0).expect("valid height");
        let unsigned_tx = Transaction{
            version: bitcoin::transaction::Version(2),
            lock_time : locktime.into(),
            input : txin_vec.clone(),
            output : txout_vec.clone(),
        };
        let mut serialized_tx = Vec::new();
        let _ = unsigned_tx.consensus_encode(&mut serialized_tx);
        let mut fee_est : f64 = 0.0;
        let dict = match &self.fee_estimates{
            Some(dict) => dict,
            None => return 3,//Need to sync error
        };
        if let  Some(value) = dict.get(&number_of_blocks.to_string()) {
            fee_est = *value;
        }
        fee_est = fee_est*((serialized_tx.len() as f64) + (txin_vec.len() as f64)*72.0);
        let fee_int = fee_est as i32;
        let mut fee_64 : u64 = fee_int as u64;
        return fee_64;
    }
    pub fn set_trusted_pending(&mut self, utxo_vec : Vec<String>){
        self.trusted_pending = Some(utxo_vec);
    }
    pub fn get_trusted_pending(&self) -> Vec<String>{
        match &self.trusted_pending{
            Some(vec) => return vec.to_vec(),
            None => return Vec::new(),
        }
    }
    //Getters
    pub fn balance(&self) -> u64{
        return self.btc;
    }
    pub fn unconfirmed_balance(&self) -> u64{
        return self.unconfirmed;
    }
    pub fn address(&self)->String{
        return self.new_address("m/0/0");
    }
    pub fn new_address(&self, derivation_path : &str)-> String{
        let network = self.get_network();
        let xpub_str = convert_to_xpub(self.xpub.clone());
        let xpub = match Xpub::from_str(&xpub_str){
            Ok(xpub) => xpub,
            Err(_) => return "Error: Invalid extended public key.".to_string(),
        };
        let derivation_path = DerivationPath::from_str(derivation_path).unwrap();
        let derived_xpub = match xpub.derive_pub(&bitcoin::secp256k1::Secp256k1::new(), &derivation_path){
            Ok(derived_xpub) => derived_xpub,
            Err(_) => return "Error: Xpub derivation error.".to_string(),
        };
        let public_key = derived_xpub.to_pub();
        let address = Address::p2wpkh(&public_key, network);
        return format!("{:?}",address);
    }
    //Helpers
    fn get_network(&self)->Network{
        if self.network =="bitcoin"{
            return Network::Bitcoin;
        }else{ //Default to testnet if specification is wrong
            return Network::Testnet;
        }
    }
    async fn fetch_fee_estimates(&self) -> Result<HashMap<String,f64>,&str>{
        let esplora_server_url = &self.esplora_url;
        let mut path = String::new();
        path.push_str(&esplora_server_url);
        path.push_str("/fee-estimates");
        let fee_histo_text  = match reqwest::get(path).await{
            Ok(fee_histo_text) => fee_histo_text,
            Err(_) => return Err("Error: Connection error, esplora url."),
        };
        let fee_histo = match fee_histo_text.text().await{
            Ok(fee_histo) => fee_histo,
            Err(_) => return Err("Error: Failed to parse result."),
        };
        let dict : HashMap<String, f64>   = serde_json::from_str(&fee_histo).unwrap();
        return Ok(dict);
    }
}

//Helper functions
pub fn convert_to_xpub(xpub_str : String) -> String{
    let zpub_bytes = bs58::decode(&xpub_str).with_check(None).into_vec().unwrap();
    let new_bytes = &zpub_bytes[4..];
    let new_prefix = hex_to_vec("0488b21e").unwrap();
    let mut vec = Vec::from(new_bytes);
    for i in (0..new_prefix.len()).rev() {
        vec.insert(0, new_prefix[i]);
    }
    return bs58::encode(vec).with_check().into_string();
}
pub fn convert_to_outpoint(utxo_str : &String) -> OutPoint{
    let parts : Vec<&str> = utxo_str.split(":").collect();
    let vout : u32 =  parts[1].parse().unwrap();
    let mut byte_arr = hex_string_to_u8_array(parts[0]).unwrap();
    byte_arr.reverse();
    let txid = Txid::from_slice(&byte_arr).expect("Error: Invalid transaction id.");
    let outpoint = OutPoint{
        txid : txid,
        vout : vout
    };
    outpoint
}
pub fn hex_string_to_u8_array(hex_str: &str) -> Result<[u8; 32], hex::FromHexError> {
    let bytes = Vec::from_hex(hex_str)?;
    if bytes.len() == 32 {
        let mut result = [0; 32];
        result.copy_from_slice(&bytes);
        Ok(result)
    } else {
        // If the length is not 32, return an error or handle the case accordingly
        Err(hex::FromHexError::InvalidStringLength)
    }
}
pub fn array_to_hex(data: &[u8]) -> String {
    let hex_string: String = data.iter()
        .map(|byte| format!("{:02x}", byte)) // Convert each byte to its hexadecimal representation
        .collect();
    hex_string
}
pub fn hex_to_vec(hex_string: &str) -> Option<Vec<u8>> {
    if hex_string.len() % 2 != 0 { return None; }
    let mut bytes = Vec::new();
    for chunk in hex_string.as_bytes().chunks(2) {
        if let Ok(byte) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16) {
            bytes.push(byte);
        }else{
            return None; 
        }
    }
    Some(bytes)
}
pub fn chunk_and_label(final_str: &str, chunk_size: usize) -> Vec<String> {
    let total_chunks = (final_str.len() + chunk_size - 1) / chunk_size; // Calculate the number of chunks
    final_str
        .chars() // Iterate over characters to respect character boundaries
        .collect::<Vec<_>>() // Collect characters into a vector for chunking
        .chunks(chunk_size) // Chunk the vector
        .enumerate() // Provide index for each chunk
        .map(|(index, chunk)| {
            let chunk_str = chunk.iter().collect::<String>(); // Convert chunk to string
            format!("({}/{}){}", index, total_chunks, chunk_str) // Format with index and total
        })
        .collect() // Collect into a vector of strings
}
pub fn base64_to_hex(base64_input: &str) -> Result<String, &'static str> {
    let bytes = base64::decode(base64_input).map_err(|_| "Error: Invalid base64 input.")?;
    let hex_string = hex::encode(bytes);
    Ok(hex_string)
}
pub fn extract_u16s(input: &str) -> Result<(u16, u16), &'static str> {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 3 {
            return Err("Error: Invalid format.");
        }
        let first_u16 = parts[1].parse::<u16>().map_err(|_| "Error: Failed to parse first number.")?;
        let second_u16 = parts[2].parse::<u16>().map_err(|_| "Error: Failed to parse second number.")?;
        Ok((first_u16, second_u16))
}
pub fn append_integers_as_bytes(vec: &mut Vec<u8>, addressdepth: u16, changedepth: u16, amount: u64) {
    let addressdepth_bytes = addressdepth.to_le_bytes();
    let changedepth_bytes = changedepth.to_le_bytes();
    let amount_bytes = amount.to_le_bytes();
    vec.extend_from_slice(&addressdepth_bytes);
    vec.extend_from_slice(&changedepth_bytes);
    vec.extend_from_slice(&amount_bytes);
}
pub fn generate_variations(input: &str) -> Vec<String> {
    let parts: Vec<&str> = input.split('/').collect();

    // Parse the maximum values
    let max1: usize = parts[1].parse().unwrap();
    let max2: usize = parts[2].parse().unwrap();

    let mut variations = Vec::new();

    // Generate all combinations
    for i in 0..=max1 {
        for j in 0..=max2 {
            variations.push(format!("m/{}/{}", i, j));
        }
    }

    variations
}

//Structs
#[derive(Debug, Serialize, Deserialize,Default,Clone)]
pub struct Utxo{
    pub utxo : String,
    pub btc : u64,
    pub txid : String,
    pub confirmed : bool,
    pub derivation_path : String,
}
#[derive(Debug, Serialize, Deserialize,Default,Clone)]
pub struct EsploraUtxos{
    pub txid : String,
    pub vout : u64,
    pub status : EsploraStatus,
    pub value : u64,
}
#[derive(Debug, Serialize, Deserialize,Default,Clone)]
pub struct EsploraStatus{
    pub confirmed : bool,
    pub block_height : Option<u64>,
    pub block_hash : Option<String>,
}