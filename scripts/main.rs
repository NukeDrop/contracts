use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{AssetId, Address, ContractId},
};
use dotenv::dotenv;
use std::{env, str::FromStr};

use memecoins_sdk::{TokenFactoryContract, FeeInfo};

#[tokio::main]
async fn main() {
    // Get command line arguments  
    let args: Vec<String> = env::args().collect();

    // Check if we have at least one argument  
    if args.len() < 2 {  
        eprintln!("Usage: cargo run -- <command> [<params>...]");  
        eprintln!("Commands: deploy <fee_amount> <fee_address> | set-fee-info <contract_id> <fee_amount> <fee_address>");  
        return;  
    }

    // Match the first argument to determine the command  
    match args[1].as_str() {  
        "deploy" => {
            if args.len() < 4 {  
                eprintln!("Usage: cargo run -- deploy <fee_amount> <fee_address>");  
                return;  
            }  
            // Call deploy function  
            deploy(args[2].as_str(), args[3].as_str()).await;  
        }  
        "set-fee-info" => {  
            if args.len() < 5 {  
                eprintln!("Usage: cargo run -- set-fee-info <contract_id> <fee_amount> <fee_address>");  
                return;  
            }  
            // Call set_fee_info function  
            set_fee_info(args[2].as_str(), args[3].as_str(), args[4].as_str()).await;
        }  
        _ => {  
            eprintln!("Unknown command: {}", args[1]);  
            eprintln!("Available commands: deploy | set-fee-info");  
        }  
    }  
}

async fn deploy(fee_amount: &str, fee_address: &str) {
    dotenv().ok();
    
    let secret = env::var("SECRET").expect("can't find SECRET");
    let node_url = env::var("FUEL_NODE_URL").expect("can't find FUEL_NODE_URL");
    let asset_id = env::var("ASSET_ID").expect("can't find ASSET_ID");

    // get wallet from private key and provider
    let provider = Provider::connect(node_url).await.unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    // fee info
    let fee_info = FeeInfo {
        fee_asset: AssetId::from_str(&asset_id).unwrap(),
        fee_amount: fee_amount.parse::<u64>().unwrap(),
        fee_address: Address::from_str(fee_address).unwrap(),
    };

    // deploy the contract
    let result = TokenFactoryContract::deploy(&wallet, &fee_info).await;
    
    let contract = result.unwrap();

    println!("Contract address = {}", contract.contract_id().hash());
    println!("Contract address = {:?}", contract.contract_id().to_string());
}

async fn set_fee_info(contract_id: &str, fee_amount: &str, fee_address: &str) {
    // import environment values
    dotenv().ok();

    let secret = env::var("SECRET").expect("can't find SECRET");
    let asset_id = env::var("ASSET_ID").expect("can't find ASSET_ID");
    let node_url = env::var("FUEL_NODE_URL").expect("can't find FUEL_NODE_URL");

    // get wallet from private key and provider
    let provider = Provider::connect(node_url).await.unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
 
    // import the contract
    let contract_id = &ContractId::from_str(contract_id).unwrap();
    let contract = TokenFactoryContract::new(contract_id.to_owned(), wallet.clone());

    //new fee info value
    let new_fee_info = FeeInfo {
        fee_asset: AssetId::from_str(&asset_id).unwrap(),
        fee_amount: fee_amount.parse::<u64>().unwrap(),
        fee_address: Address::from_str(fee_address).unwrap(),
    };
    
    //set fee info
    let _ = contract.set_fee_info(new_fee_info).await;
    let result = contract.fee_info().await.unwrap().value;
    
    println!("Fee asset: {:?}", result.fee_asset);
    println!("Fee address: {:?}", result.fee_address);
    println!("Fee amount: {:?}", result.fee_amount);
}