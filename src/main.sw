contract;

mod errors;
mod events;

use standards::{
    src20::SRC20, 
    src7::{Metadata, SRC7}, 
    src3::SRC3, 
    src5::{SRC5, State},
};
use std::{
    hash::{Hash, sha256}, 
    storage::storage_string::*,
    storage::storage_vec::*, 

    string::String, 
    option::Option, 
    asset::mint_to,
};
use sway_libs::{
    asset::{
        metadata::*,
        base::{
            _decimals,
            _name,
            _set_decimals,
            _set_name,
            _set_symbol,
            _symbol,
            _total_assets,
            _total_supply,
        },
    },
    ownership::{
        _owner,
        initialize_ownership,
        transfer_ownership,
    },
};
use errors::*;
use events::*;

storage {
    total_assets: u64 = 0,
    total_supply: StorageMap<AssetId, u64> = StorageMap {},
    name: StorageMap<AssetId, StorageString> = StorageMap {},
    symbol: StorageMap<AssetId, StorageString> = StorageMap {},
    decimals: StorageMap<AssetId, u8> = StorageMap {},
    logo: StorageMap<AssetId, StorageString> = StorageMap {},
    description: StorageMap<AssetId, StorageString> = StorageMap {},
    metadata: StorageMetadata = StorageMetadata {},
    asset: StorageMap<b256, AssetId> = StorageMap {},
}

abi TokenFactory {
    #[storage(read, write)]
    fn new_asset(
        name: String,
        symbol: String,
        decimals: u8,
        mint_amount: u64,
        logo: Option<String>,
        description: Option<String>,
        metadata_list: Vec<(String, Metadata)>,
    ) -> AssetId;

    #[storage(read)]
    fn get_asset(symbol: String) -> Option<AssetId>;

    #[storage(read, write)]
    fn initialize_ownership(new_owner: Identity);

    #[storage(read, write)]
    fn transfer_ownership(new_owner: Identity);
}

impl TokenFactory for Contract {
    #[storage(read, write)]
    fn initialize_ownership(new_owner: Identity) {
        initialize_ownership(new_owner);
    }

    #[storage(read, write)]
    fn transfer_ownership(new_owner: Identity) {
        transfer_ownership(new_owner);
    }

    #[storage(read, write)]
    fn new_asset(
        name: String,
        symbol: String,
        decimals: u8,
        mint_amount: u64,
        logo: Option<String>,
        description: Option<String>,
        metadata_list: Vec<(String, Metadata)>,
    ) -> AssetId {
        // validate the token mint amount
        require(mint_amount > 0, TokenError::ZeroMintAmount);

        // validate the token name
        require(
            name
                .as_bytes()
                .len() > 0 && name
                .as_bytes()
                .len() < 33,
            TokenError::InvalidName(name),
        );

        // validate the token symbol
        require(
            symbol
                .as_bytes()
                .len() > 0 && symbol
                .as_bytes()
                .len() < 11,
            TokenError::InvalidSymbol(symbol),
        );

        // initialize the ownership
        let owner = msg_sender().unwrap();
        initialize_ownership(owner);

        // generate a sub id of the token
        let sub_id = sha256((ContractId::this(), symbol));
        let asset = AssetId::new(ContractId::this(), sub_id);
        
        // check if the asset already existed
        require(storage.total_supply.get(asset).try_read().is_none(), TokenError::TokenAlreadyExists(asset));

        _set_name(storage.name, asset, name);
        _set_symbol(storage.symbol, asset, symbol);
        _set_decimals(storage.decimals, asset, decimals);

        storage.total_assets.write(storage.total_assets.read() + 1);
        storage.total_supply.insert(asset, mint_amount);

        // mint the tokens to the token creator
        mint_to(owner, sub_id, mint_amount);

        storage.logo.insert(asset, StorageString {});
        if let Some(logo_str) = logo {
            storage.logo.get(asset).write_slice(logo_str);
        }

        storage.description.insert(asset, StorageString {});
        if let Some(description_str) = description {
            storage.description.get(asset).write_slice(description_str);
        }

        storage.asset.insert(sha256(symbol), asset);

        let len = metadata_list.len();
        require(len < 7, TokenError::TooManyTags);

        let mut i = 0;

        while i < len {
            let metadata = metadata_list.get(i).unwrap();
            _set_metadata(storage.metadata, asset, metadata.0, metadata.1);
            i += 1;
        }

        log(AssetNew {
            asset,
            owner,
            name,
            symbol,
            decimals,
            supply: mint_amount,
            logo,
            description,
            tags: metadata_list,
        });
        asset
    }

    #[storage(read)]
    fn get_asset(symbol: String) -> Option<AssetId> {
        storage.asset.get(sha256(symbol)).try_read()
    }
}

impl SRC7 for Contract {
    #[storage(read)]
    fn metadata(asset: AssetId, key: String) -> Option<Metadata> { 
        if key == String::from_ascii_str("name") {
            Some(Metadata::String(storage.name.get(asset).read_slice().unwrap()))
        } else if key == String::from_ascii_str("symbol") {
            Some(Metadata::String(storage.symbol.get(asset).read_slice().unwrap()))
        } else if key == String::from_ascii_str("decimals") {
            Some(Metadata::Int(storage.decimals.get(asset).read().into()))
        } else if key == String::from_ascii_str("logo") {
            Some(Metadata::String(storage.logo.get(asset).read_slice().unwrap())) 
        } else if key == String::from_ascii_str("description") {
            Some(Metadata::String(storage.description.get(asset).read_slice().unwrap()))
        } else {
            None
        }
    }
}

impl SRC5 for Contract {
    #[storage(read)]
    fn owner() -> State {
        _owner()
    }
}

impl SRC20 for Contract {
    #[storage(read)]
    fn total_assets() -> u64 {
        _total_assets(storage.total_assets)
    }
 
    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        storage.total_supply.get(asset).try_read()
    }

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        _name(storage.name, asset)
    }

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        _symbol(storage.symbol, asset)
    }

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        _decimals(storage.decimals, asset)
    }
}

