use fuels::{
    prelude::{
        abigen, Contract, LoadConfiguration, StorageConfiguration, TxPolicies,
        CallParameters, WalletUnlocked, VariableOutputPolicy,
    },
    programs::responses::CallResponse,
    types::{bech32::Bech32ContractId, AssetId, Bytes32, Identity, ContractId},
};

use rand::Rng;
use std::path::PathBuf;

// Load abi from json
abigen!(Contract(name = "TokenFactory", abi = "contract/out/release/memecoins-contract-abi.json"));

const TOKENFACTORY_CONTRACT_BINARY_PATH: &str = "contract/out/release/memecoins-contract.bin";
const TOKENFACTORY_CONTRACT_STORAGE_PATH: &str =
    "contract/out/release/memecoins-contract-storage_slots.json";

pub struct TokenFactoryContract {
    instance: TokenFactory<WalletUnlocked>,
}

impl TokenFactoryContract{
    pub async fn deploy(wallet: &WalletUnlocked, fee_info: &FeeInfo) -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(root.join(TOKENFACTORY_CONTRACT_STORAGE_PATH));

        let contract_configuration =
            LoadConfiguration::default().with_storage_configuration(storage_configuration?);

        let contract_id = Contract::load_from(
            root.join(TOKENFACTORY_CONTRACT_BINARY_PATH),
            contract_configuration,
        )?
        .with_salt(salt)
        .deploy(wallet, TxPolicies::default())
        .await?;

        let tokenfactory = TokenFactory::new(contract_id.clone(), wallet.clone());

        let _self = Self {
            instance: tokenfactory,
        };

        _self.initialize(wallet.address().into(), fee_info).await?;

        Ok(_self)
    }

    // Initialize the contract with contract ID and wallet
    pub fn new(contract_id: ContractId, wallet: WalletUnlocked) -> Self {
        // Create a new contract instance
        let instance = TokenFactory::new(contract_id, wallet);

        TokenFactoryContract { instance }
    }

    pub async fn with_account(&self, account: &WalletUnlocked) -> anyhow::Result<Self> {
        Ok(Self {
            instance: self.instance.clone().with_account(account.clone()),
        })
    }

    pub fn id(&self) -> Bytes32 {
        self.instance.contract_id().hash
    }

    pub fn contract_id(&self) -> &Bech32ContractId {
        self.instance.contract_id()
    }

    pub async fn initialize(
        &self,
        owner: Identity,
        fee_info: &FeeInfo,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .initialize(owner, fee_info.to_owned())
            .call()
            .await?)
    }

    pub async fn transfer_ownership(
        &self,
        recipient: Identity,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .transfer_ownership(recipient)
            .call()
            .await?)
    }

    pub async fn owner(&self) -> anyhow::Result<CallResponse<State>> {
        Ok(self.instance.methods().owner().call().await?)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn new_asset(
        &self,         
        name: String,
        symbol: String,
        decimals: u8,
        mint_amount: u64,
        logo: Option<String>,
        description: Option<String>,
        metadata_list: Vec<(String, Metadata)>,
        fee_asset: AssetId,
        amount: u64,
        gas: u64,
    ) -> anyhow::Result<CallResponse<AssetId>> {
        let tx_policies = TxPolicies::new(Some(0), None, Some(0), None, Some(gas));
        let call_params = CallParameters::new(amount, fee_asset, gas);
        Ok(self
            .instance
            .methods()
            .new_asset(name, symbol, decimals, mint_amount, logo, description, metadata_list)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(2))
            .with_tx_policies(tx_policies)
            .call_params(call_params)
            .expect("Call param Error")
            .call()
            .await?)
    }

    pub async fn total_assets(&self) -> anyhow::Result<CallResponse<u64>> {
        Ok(self.instance.methods().total_assets().call().await?)
    }

    pub async fn total_supply(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u64>>> {
        Ok(self
            .instance
            .methods()
            .total_supply(*asset)
            .call()
            .await?)
    }

    pub async fn name(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<String>>> {
        Ok(self.instance.methods().name(*asset).call().await?)
    }

    pub async fn symbol(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<String>>> {
        Ok(self.instance.methods().symbol(*asset).call().await?)
    }

    pub async fn decimals(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u8>>> {
        Ok(self
            .instance
            .methods()
            .decimals(*asset)
            .call()
            .await?)
    }

    pub async fn get_asset(&self, name: &String) -> anyhow::Result<CallResponse<Option<AssetId>>> {
        Ok(self
            .instance
            .methods()
            .get_asset(name.to_owned())
            .call()
            .await?)
    }

    pub async fn fee_info(&self) -> anyhow::Result<CallResponse<FeeInfo>> {
        Ok(self.instance.methods().get_fee_info().call().await?)
    }

    pub async fn set_fee_info(&self, fee_info: FeeInfo) -> anyhow::Result<CallResponse<()>> {
        Ok(self.instance.methods().set_fee_info(fee_info).call().await?)
    }

    pub async fn metadata(&self, asset: AssetId, key: String) -> anyhow::Result<CallResponse<Option<Metadata>>> {
        Ok(self.instance.methods().metadata(asset, key).call().await?)
    }
}