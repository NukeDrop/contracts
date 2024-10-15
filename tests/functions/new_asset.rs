use crate::setup::setup;

use fuels::{
    types::AssetId,
    accounts::ViewOnlyAccount,
};
use memecoins_sdk::{AssetNew, Metadata};

mod success {
    use super::*;

    #[tokio::test]
    async fn create_asset() -> anyhow::Result<()> {
        let (contract, owner, user) = setup().await?;

        let name = String::from("BTC_NAME");
        let symbol = String::from("BTC");
        let decimals = 8;
        let mint_amount = 1000000;
        let fee_asset = AssetId::zeroed();
        let amount = 10u64;
        let gas = 200_000;
        let logo = Some(String::from("https://example.com/logo.png"));
        let description = Some(String::from("meme coin"));
        let metadata = Some(vec![
            ("key1".to_string(), Metadata::String("value1".to_string())),
            ("key2".to_string(), Metadata::String("value2".to_string())),
            ("key3".to_string(), Metadata::String("value3".to_string())),
            ("twitter".to_string(), Metadata::String("https://twitter.com/example".to_string())),
            ("discord".to_string(), Metadata::String("https://discord.gg/example".to_string())),
        ]);

        let wallet_balance = owner.wallet.get_asset_balance(&fee_asset).await?;
        let response = contract
            .with_account(&user.wallet)
            .await?
            .new_asset(name.clone(), symbol.clone(), decimals, mint_amount, logo.clone(), description.clone(), metadata.clone(), fee_asset, amount, gas)
            .await?;
        let asset = response.value;
        assert_ne!(response.value, AssetId::zeroed());

        // check if the fee asset paid by user is sent to the owner of the contract
        assert_eq!(owner.wallet.get_asset_balance(&fee_asset).await?, wallet_balance + amount);

        // check if the token is minted to the token creator
        assert_eq!(user.wallet.get_asset_balance(&asset).await?, mint_amount);

        assert_eq!(contract.total_assets().await?.value, 1);
        assert_eq!(contract.total_supply(&asset).await?.value, Some(mint_amount));
        assert_eq!(contract.name(&asset).await?.value, Some(name.clone()));
        assert_eq!(contract.symbol(&asset).await?.value, Some(symbol.clone()));
        assert_eq!(contract.decimals(&asset).await?.value, Some(decimals));
        assert_eq!(contract.get_asset(&symbol).await?.value, Some(asset));

        // check the metadata
        assert_eq!(contract.metadata(asset, "logo".to_string()).await?.value, Some(Metadata::String(logo.clone().unwrap())));
        assert_eq!(contract.metadata(asset, "description".to_string()).await?.value, Some(Metadata::String(description.clone().unwrap())));
        assert_eq!(contract.metadata(asset, "key1".to_string()).await?.value, Some(Metadata::String("value1".to_string())));
        assert_eq!(contract.metadata(asset, "key2".to_string()).await?.value, Some(Metadata::String("value2".to_string())));
        assert_eq!(contract.metadata(asset, "key3".to_string()).await?.value, Some(Metadata::String("value3".to_string())));
        assert_eq!(contract.metadata(asset, "twitter".to_string()).await?.value, Some(Metadata::String("https://twitter.com/example".to_string())));
        assert_eq!(contract.metadata(asset, "discord".to_string()).await?.value, Some(Metadata::String("https://discord.gg/example".to_string())));

        // check if the event is emitted properly
        let log = response.decode_logs_with_type::<AssetNew>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            AssetNew {
                asset,
                owner: user.wallet.address().into(),
                name,
                symbol,
                decimals,
                supply: mint_amount,
                logo,
                description,
                tags: metadata,
            },
        );

        Ok(())
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "FeeAmountInsufficient")]
    async fn create_asset_fee_insufficient() {
        let (contract, _owner, user) = setup().await.unwrap();

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;
        let mint_amount = 1000000;
        let amount = 5u64;
        let gas = 200_000;
        let fee_asset = AssetId::zeroed();

        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .new_asset(name.clone(), symbol.clone(), decimals, mint_amount, None, None, None, fee_asset, amount, gas)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidName")]
    async fn create_asset_invalid_name() {
        let (contract, _owner, user) = setup().await.unwrap();

        let name = String::from("THIS_TOKEN_NAME_LENGTH_LARGER_THAN_32");
        let symbol = String::from("BTC");
        let decimals = 8;
        let mint_amount = 1000000;
        let fee_asset = AssetId::zeroed();
        let amount = 10u64;
        let gas = 200_000;

        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .new_asset(name.clone(), symbol.clone(), decimals, mint_amount, None, None, None, fee_asset, amount, gas)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidSymbol")]
    async fn create_asset_invalid_symbol() {
        let (contract, _owner, user) = setup().await.unwrap();

        let name = String::from("BTC");
        let symbol = String::from("SYMBOL_LENGTH_LONG");
        let decimals = 8;
        let mint_amount = 1000000;
        let fee_asset = AssetId::zeroed();
        let amount = 10u64;
        let gas = 200_000;

        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .new_asset(name.clone(), symbol.clone(), decimals, mint_amount, None, None, None, fee_asset, amount, gas)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "ZeroMintAmount")]
    async fn create_asset_with_zero_mint_amount() {
        let (contract, _owner, user) = setup().await.unwrap();

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;
        let mint_amount = 0;
        let fee_asset = AssetId::zeroed();
        let amount = 10u64;
        let gas = 200_000;

        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .new_asset(name.clone(), symbol.clone(), decimals, mint_amount, None, None, None, fee_asset, amount, gas)
            .await
            .unwrap();
    }
}