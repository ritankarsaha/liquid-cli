use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: String,
    method: String,
    params: Value,
}

#[derive(Deserialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcError>,
}

#[derive(Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

struct RpcClient {
    client: Client,
    url: String,
}

impl RpcClient {
    fn new(url: &str) -> Self {
        let client = Client::new();
        RpcClient { client, url: url.to_string() }
    }
    
    fn send_request<T>(&self, method: &str, params: Value) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.to_string(),
            params,
        };
        
        let response = self.client
            .post(&self.url)
            .json(&request)
            .send()?
            .json::<RpcResponse>()?;
        
        if let Some(error) = response.error {
            return Err(anyhow::anyhow!("RPC error ({}): {}", error.code, error.message));
        }
        
        let result = response.result.context("No result in RPC response")?;
        let result: T = serde_json::from_value(result)?;
        
        Ok(result)
    }
}

fn get_rpc_url() -> String {
    // We need to replace this with our actual RPC credentials
    "http://yourusername:yourpassword@127.0.0.1:18891/wallet/ritankar_wallet".to_string()
}

pub fn run_transfer_poc() -> Result<()> {
    println!("Starting Liquid Asset Transfer Proof of Concept");
    println!("-----------------------------------------------");
    
    let client = RpcClient::new(&get_rpc_url());
    
    //Generating two addresses
    println!("\nStep 1: Generating two addresses");
    let params = serde_json::json!([]);
    let address1: String = client.send_request("getnewaddress", params)?;
    println!("Address 1: {}", address1);
    
    let params = serde_json::json!([]);
    let address2: String = client.send_request("getnewaddress", params)?;
    println!("Address 2: {}", address2);
    
    //Checking if we have any assets
    println!("\nStep 2: Checking for existing assets");
    let params = serde_json::json!([]);
    let balances: Value = client.send_request("getbalance", params)?;
    
    let mut asset_id = String::new();
    let mut has_balance = false;
    
    if let Some(obj) = balances.as_object() {
        for (id, amount) in obj {
            if amount.as_f64().unwrap_or(0.0) > 0.0 {
                asset_id = id.clone();
                has_balance = true;
                println!("Found asset with balance: {} ({})", id, amount);
                break;
            }
        }
    }
    
    //If we don't have any assets with balance,we will be trying to issue a new one
    if !has_balance {
        println!("\nStep 3: No assets with balance found. Attempting to issue a new asset...");
        match client.send_request::<Value>("issueasset", serde_json::json!([100, 1])) {
            Ok(result) => {
                asset_id = result["asset"].as_str().unwrap_or("").to_string();
                println!("Successfully issued new asset: {}", asset_id);
                
                // Waiting a moment for the transaction to be processed
                println!("Waiting for transaction to be processed...");
                sleep(Duration::from_secs(2));
            },
            Err(e) => {
                println!("Failed to issue asset: {}", e);
                println!("Will try to use the bitcoin asset for demonstration");
                
                // Getting the bitcoin asset ID
                let params = serde_json::json!([]);
                let asset_labels: Value = client.send_request("dumpassetlabels", params)?;
                if let Some(bitcoin) = asset_labels["bitcoin"].as_str() {
                    asset_id = bitcoin.to_string();
                    println!("Using bitcoin asset: {}", asset_id);
                } else {
                    return Err(anyhow::anyhow!("No assets available for transfer demonstration"));
                }
            }
        }
    }
    
    //Transfering a small amount to the second address
    println!("\nStep 4: Transferring asset to the second address");
    println!("Asset ID: {}", asset_id);
    println!("From: {}", address1);
    println!("To: {}", address2);
    
    // Creating a map for the outputs
    let mut outputs = HashMap::new();
    outputs.insert(address2.clone(), 0.1);
    
let params = serde_json::json!([
    address2,           
    0.1,               
    "",                
    "",                 
    false,              
    false,             
    1,                 
    "UNSET",            
    null,             
    asset_id,           
    true,             
    null,              
    false          
]);
    
    match client.send_request::<String>("sendtoaddress", params) {
        Ok(txid) => {
            println!("Transfer successful!");
            println!("Transaction ID: {}", txid);
            
            // Waiting a moment for the transaction to be processed
            println!("Waiting for transaction to be processed...");
            sleep(Duration::from_secs(2));
            
            // Verifying the transfer
            println!("\nStep 5: Verifying the transfer");
            let params = serde_json::json!([]);
            let balances: Value = client.send_request("getbalance", params)?;
            
            println!("Updated balances:");
            if let Some(obj) = balances.as_object() {
                for (id, amount) in obj {
                    println!("  Asset {}: {}", id, amount);
                }
            }
            
            // Getting transaction details
            let params = serde_json::json!([txid, true]);
            let tx_details: Value = client.send_request("gettransaction", params)?;
            
            println!("\nTransaction details:");
            println!("  Confirmations: {}", tx_details["confirmations"].as_i64().unwrap_or(0));
            println!("  Amount: {}", tx_details["amount"].as_f64().unwrap_or(0.0));
            println!("  Fee: {}", tx_details["fee"].as_f64().unwrap_or(0.0));
            
            println!("\nProof of concept completed successfully!");
        },
        Err(e) => {
            println!("Transfer failed: {}", e);
            println!("\nAlternative demonstration: Simulating a transfer");
            
            // Simulating a transfer for demonstration purposes only in this case
            // Transactions are failing due to insufficient funds in this case.
            println!("Simulated transfer of asset {} from {} to {}", asset_id, address1, address2);
            println!("Simulated transaction ID: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");
            
        }
    }
    
    Ok(())
}
