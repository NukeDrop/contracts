library;

use std::string::String;
use standards::src7::Metadata;

pub struct AssetNew {
    pub asset: AssetId,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub owner: Identity,
    pub supply: u64,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<(String, Metadata)>>,
}