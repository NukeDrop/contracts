use crate::setup::setup;

use fuels::types::{AssetId, Address};
use memecoins_sdk::{FeeInfo, State};

mod success {

    use super::*;

    #[tokio::test]
    async fn transfer_ownership() -> anyhow::Result<()> {
        let (contract, owner, user) = setup().await?;
        let new_fee_info = FeeInfo {
            fee_asset: AssetId::zeroed(),
            fee_amount: 1000,
            fee_address: Address::zeroed(),
        };
        contract
            .with_account(&owner.wallet)
            .await?
            .transfer_ownership(user.wallet.address().into())
            .await?;

        assert_eq!(contract.owner().await?.value, State::Initialized(user.wallet.address().into()));

        contract
            .with_account(&user.wallet)
            .await?
            .set_fee_info(new_fee_info)
            .await?;

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn transfer_ownership_not_owner() {
        let (contract, _, user) = setup().await.unwrap();

        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .transfer_ownership(user.wallet.address().into())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn transfer_ownership_verify_old_owner() {
        let (contract, owner, user) = setup().await.unwrap();

        let new_fee_info = FeeInfo {
            fee_asset: AssetId::zeroed(),
            fee_amount: 1000,
            fee_address: Address::zeroed(),
        };

        contract
            .with_account(&owner.wallet)
            .await
            .unwrap()
            .transfer_ownership(user.wallet.address().into())
            .await
            .unwrap();

        contract
            .with_account(&owner.wallet)
            .await
            .unwrap()
            .set_fee_info(new_fee_info)
            .await
            .unwrap();
    }
}