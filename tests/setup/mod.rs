use fuels::prelude::{
    launch_custom_provider_and_get_wallets, WalletUnlocked, WalletsConfig, DEFAULT_COIN_AMOUNT, AssetId,
};

use memecoins_sdk::{TokenFactoryContract, FeeInfo};

pub(crate) struct User {
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) async fn setup() -> anyhow::Result<(TokenFactoryContract, User, User)> {
    let config = WalletsConfig::new(Some(2), Some(1), Some(DEFAULT_COIN_AMOUNT));
    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await?;
    let deployer = wallets.pop().unwrap();
    let user = wallets.pop().unwrap();

    let fee_info = FeeInfo {
        fee_asset: AssetId::zeroed(),
        fee_amount: 10,
        fee_address: deployer.address().into(),
    };

    let contract = TokenFactoryContract::deploy(&deployer, &fee_info).await?;

    let deployer = User { wallet: deployer };
    let user = User { wallet: user };

    Ok((contract, deployer, user))
}