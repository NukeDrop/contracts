library;

use std::string::String;

pub enum TokenError {
    InvalidAssetPayment: (),
    FeeAmountInsufficient: (),
    InvalidName: (String),
    InvalidSymbol: (String),
    ZeroMintAmount: (),
    TokenAlreadyExists: (AssetId),
    TooManyTags: (),
}
