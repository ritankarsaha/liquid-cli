mod transfer_poc;
use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    TransferPoc,

    // Generating a new Liquid address
    Address {
        /// Generating a new address
        #[arg(short, long)]
        new: bool,
    },
    // Displaying information about Liquid assets
    Asset {
        /// Asset ID to query (optional)
        #[arg(short, long)]
        id: Option<String>,
    },
    // Transfering assets between Liquid addresses
    Transfer {
        // Recipient address
        #[arg(short, long)]
        to: String,
        // Asset ID (default is L-BTC)
        #[arg(short, long)]
        asset: Option<String>,
        // Amount to send
        #[arg(short, long)]
        amount: f64,
    },
    // Creating or loading a wallet
    Wallet {
        // Creating a new wallet with the given name
        #[arg(short, long)]
        create: Option<String>,

        // Loading an existing wallet with the given name
        #[arg(short = 'o', long)] 
        load: Option<String>,

        // Listing available wallets
        #[arg(short, long)]
        list: bool,
    },
}

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
        
        let result = response.result.ok_or_else(|| anyhow::anyhow!("No result in RPC response"))?;
        let result: T = serde_json::from_value(result)?;
        
        Ok(result)
    }
}

fn get_rpc_url() -> String {
    // We need to replace with our actual RPC credentials from elements-testnet.conf and also tweak the link properly to add our wallet to this.

    "http://yourusername:yourpassword@127.0.0.1:18891/wallet/ritankar_wallet".to_string()
}

fn ensure_wallet_loaded() -> Result<()> {
    let client = RpcClient::new(&get_rpc_url());
    
    // Checking if any wallet is loaded
    let params = serde_json::json!([]);
    let result: Value = client.send_request("listwallets", params)?;
    
    let wallets = result.as_array().unwrap();
    if wallets.is_empty() {

        // No wallets loaded, therefore trying to load the default wallet
        println!("No wallet is currently loaded. Attempting to load or create a default wallet...");
        
        // Trying to load the default wallet
        let params = serde_json::json!(["default"]);
        match client.send_request::<Value>("loadwallet", params) {
            Ok(_) => {
                println!("Successfully loaded the default wallet.");
                return Ok(());
            },
            Err(_) => {

                // Default wallet does not exist, so we create it
                println!("Default wallet not found. Creating a new default wallet...");
                let params = serde_json::json!(["default"]);
                match client.send_request::<Value>("createwallet", params) {
                    Ok(_) => {
                        println!("Successfully created and loaded the default wallet.");
                        return Ok(());
                    },
                    Err(e) => {
                        return Err(anyhow::anyhow!("Failed to create default wallet: {}", e));
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn handle_wallet_command(create: Option<String>, load: Option<String>, list: bool) -> Result<()> {
    let client = RpcClient::new(&get_rpc_url());
    
    if list {

        // Listing all available wallets
        let params = serde_json::json!([]);
        let result: Value = client.send_request("listwalletdir", params)?;
        
        println!("Available wallets:");
        if let Some(wallets) = result["wallets"].as_array() {
            if wallets.is_empty() {
                println!("  No wallets found");
            } else {
                for (i, wallet) in wallets.iter().enumerate() {
                    println!("  {}: {}", i + 1, wallet["name"].as_str().unwrap_or("Unknown"));
                }
            }
        } else {
            println!("  No wallets found");
        }
        
        // Listing all loaded wallets

        let params = serde_json::json!([]);
        let loaded_wallets: Vec<String> = client.send_request("listwallets", params)?;
        
        println!("\nCurrently loaded wallets:");
        if loaded_wallets.is_empty() {
            println!("  No wallets currently loaded");
        } else {
            for (i, wallet) in loaded_wallets.iter().enumerate() {
                println!("  {}: {}", i + 1, wallet);
            }
        }
    } else if let Some(name) = create {

        // Creating a new wallet
        let params = serde_json::json!([name]);
        let _: Value = client.send_request("createwallet", params)?;
        println!("Successfully created and loaded wallet: {}", name);
    } else if let Some(name) = load {

        // Loading an existing wallet
        let params = serde_json::json!([name]);
        let _: Value = client.send_request("loadwallet", params)?;
        println!("Successfully loaded wallet: {}", name);
    } else {
        println!("Please specify an action: --create, --load, or --list");
    }
    
    Ok(())
}

fn handle_address_command(new: bool) -> Result<()> {
    // Ensuring a wallet is loaded before proceeding
    ensure_wallet_loaded()?;
    
    let client = RpcClient::new(&get_rpc_url());
    
    if new {
        // Generating a new address
        let params = serde_json::json!([]);
        let address: String = client.send_request("getnewaddress", params)?;
        println!("New Liquid address: {}", address);
    } else {
        // List ethe xisting addresses
        let params = serde_json::json!([""]);
        let addresses: Value = client.send_request("getaddressesbylabel", params)?;
        
        if addresses.as_object().unwrap().is_empty() {
            println!("No addresses found. Generate one with 'liquid-cli address --new'");
        } else {
            println!("Existing Liquid addresses:");
            for (addr, _) in addresses.as_object().unwrap() {
                println!("  {}", addr);
            }
        }
    }
    
    Ok(())
}

fn handle_asset_command(asset_id: Option<String>) -> Result<()> {

    // Ensuring a wallet is loaded before proceeding
    ensure_wallet_loaded()?;
    
    let client = RpcClient::new(&get_rpc_url());
    
    if let Some(id) = asset_id {

        // Displaying information about a specific asset

        let params = serde_json::json!([id]);
        let asset_info: Value = client.send_request("getassetinfo", params)?;
        
        println!("Asset Information:");
        println!("  ID: {}", asset_info["asset"].as_str().unwrap_or("Unknown"));
        println!("  Name: {}", asset_info["name"].as_str().unwrap_or("Unknown"));
        println!("  Precision: {}", asset_info["precision"].as_u64().unwrap_or(0));
        println!("  Issuance Txid: {}", asset_info["issuance_txid"].as_str().unwrap_or("Unknown"));
    } else {

        // Listing all assets in the wallet
        let params = serde_json::json!([]);
        let balances: Value = client.send_request("getbalance", params)?;
        
        println!("Your Liquid Assets:");
        
        if balances.as_object().unwrap().is_empty() {
            println!("  No assets found in your wallet");
        } else {
            for (asset, amount) in balances.as_object().unwrap() {
                println!("  Asset: {}", asset);
                println!("    Balance: {}", amount);
                
                // Trying to get additional asset info
                let params = serde_json::json!([asset]);
                match client.send_request::<Value>("getassetinfo", params) {
                    Ok(info) => {
                        println!("    Name: {}", info["name"].as_str().unwrap_or("Unknown"));
                        println!("    Precision: {}", info["precision"].as_u64().unwrap_or(0));
                    },
                    Err(_) => {
                        println!("    Name: Unknown");
                        println!("    Precision: Unknown");
                    }
                }
                
                println!();
            }
        }
    }
    
    Ok(())
}

fn handle_transfer_command(to: &str, asset_id: Option<String>, amount: f64) -> Result<()> {

    // Ensuring a wallet is loaded before proceeding
    ensure_wallet_loaded()?;
    
    let client = RpcClient::new(&get_rpc_url());
    
    // Default to L-BTC if no asset is specified
    let asset = asset_id.unwrap_or_else(|| {

        // Testnet L-BTC asset ID
        "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49".to_string()
    });
    
    // Creating the transaction properly 
    let params = serde_json::json!([to, amount, "", "", false, false, 1, "UNSET", asset]);
    let txid: String = client.send_request("sendtoaddress", params)?;
    
    println!("Transaction sent successfully!");
    println!("Transaction ID: {}", txid);
    
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Address { new } => {
            handle_address_command(*new)?;
        }
        Commands::Asset { id } => {
            handle_asset_command(id.clone())?;
        }
        Commands::Transfer { to, asset, amount } => {
            handle_transfer_command(to, asset.clone(), *amount)?;
        }
        Commands::Wallet { create, load, list } => {
            handle_wallet_command(create.clone(), load.clone(), *list)?;
        }
        Commands::TransferPoc => {
            transfer_poc::run_transfer_poc()?;
        }
    }

    Ok(())
}