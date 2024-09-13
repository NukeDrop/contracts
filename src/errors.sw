library;

use std::string::String;

pub enum TokenError {
    TooManyTags: (),
    ZeroMintAmount: (),
    TokenAlreadyExists: (AssetId),
    InvalidName: (String),
    InvalidSymbol: (String),
}
