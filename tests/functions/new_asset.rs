use crate::setup::setup;

use fuels::{
    types::AssetId,
    accounts::ViewOnlyAccount,
};
use memecoins_sdk::AssetNew;

mod success {
    use super::*;

    #[tokio::test]
    async fn create_asset() -> anyhow::Result<()> {
        let (contract, owner, _user) = setup().await?;

        let name = String::from("BTC_NAME");
        let symbol = String::from("BTC");
        let decimals = 8;
        let mint_amount = 1000000;
        let amount = 50u64;
        let gas = 200_000;

        let response = contract
            .with_account(&owner.wallet)
            .await?
            .new_asset(name.clone(), symbol.clone(), decimals, mint_amount, None, None, vec![], amount, gas)
            .await?;
        let asset = response.value;
        assert_ne!(response.value, AssetId::zeroed());

        let log = response.decode_logs_with_type::<AssetNew>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            AssetNew {
                asset: asset,
                owner: owner.wallet.address().into(),
                name: name.clone(),
                symbol: symbol.clone(),
                decimals: decimals,
                supply: mint_amount,
                logo: None,
                description: None,
                tags: vec![],
            },
        );
        assert_eq!(contract.total_assets().await?.value, 1);
        assert_eq!(contract.total_supply(&asset).await?.value, Some(mint_amount));
        assert_eq!(contract.name(&asset).await?.value, Some(name.clone()));
        assert_eq!(contract.symbol(&asset).await?.value, Some(symbol.clone()));
        assert_eq!(contract.decimals(&asset).await?.value, Some(decimals));
        assert_eq!(contract.asset_get(&symbol).await?.value, Some(asset));

        assert_eq!(owner.wallet.get_asset_balance(&asset).await?, mint_amount);

        Ok(())
    }
}