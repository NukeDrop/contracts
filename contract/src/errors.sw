library;

use std::string::String;

pub enum TokenError {
    InvalidAssetPayment: (),
    FeeAmountInsufficient: (),
    InvalidName: (String),
    InvalidSymbol: (String),
    InvalidDescription: (),
    ZeroMintAmount: (),
    TokenAlreadyExists: (AssetId),
}
