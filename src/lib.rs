use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet}; 
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, Gas, PublicKey}; 
use std::collections::HashMap;
use serde_json::json; 
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata};
use near_sdk::json_types::U128;

const CODE: &[u8] = include_bytes!("../res/non_fungible_token.wasm"); 

#[near_bindgen] 
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] 
pub struct NFTFactory { 
    subaccounts: UnorderedSet<String>, 
    master_pk: PublicKey
}

const NO_DEPOSIT: u128 = 0;
const MIN_ATTACHED_BALANCE: u128 = 3_500_000_000_000_000_000_000_000;
const MAX_GAS: Gas = Gas(80_000_000_000_000);

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    SubAccounts,
}

#[near_bindgen]
impl NFTFactory {
    #[init]
    pub fn new() -> Self {
        Self {
            subaccounts: UnorderedSet::new(StorageKey::SubAccounts),
            master_pk: env::signer_account_pk()
        }
    }

    #[payable]
    pub fn create_default(&mut self, subaccount: String) {
        let attached_deposit = env::attached_deposit();
        let caller_id = env::predecessor_account_id();
        assert!(attached_deposit >= MIN_ATTACHED_BALANCE);
        assert!(
            !self.subaccounts.contains(&subaccount),
            "Error: subaccount already exist"
        );

        let contract_account_id: AccountId = AccountId::from(format!("{}.{}", subaccount, env::current_account_id()).parse().unwrap());

        self.subaccounts.insert(&subaccount);

        Promise::new(contract_account_id.clone())
            .create_account()
            .add_full_access_key(self.master_pk.clone())
            .deploy_contract(CODE.to_vec())
            .transfer(attached_deposit)
            .function_call(
                "new_default_meta".to_string(),
                json!({"owner_id": caller_id}).to_string().into_bytes(),
                NO_DEPOSIT,
                MAX_GAS
            );
    }

    #[payable]
    pub fn create(&mut self, 
        subaccount: String, 
        metadata: NFTContractMetadata, 
        token_metadata: TokenMetadata,
        minting_price: U128,
        perpetual_royalties: Option<HashMap<AccountId, u32>>
    ) {
        let attached_deposit = env::attached_deposit();
        let caller_id = env::predecessor_account_id();
        assert!(attached_deposit >= MIN_ATTACHED_BALANCE);
        assert!(
            !self.subaccounts.contains(&subaccount),
            "Error: subaccount already exist"
        );

        let contract_account_id: AccountId = AccountId::from(format!("{}.{}", subaccount, env::current_account_id()).parse().unwrap());

        self.subaccounts.insert(&subaccount);

        Promise::new(contract_account_id.clone())
            .create_account()
            .add_full_access_key(self.master_pk.clone())
            .deploy_contract(CODE.to_vec())
            .transfer(attached_deposit)
            .function_call(
                "new".to_string(),
                json!({
                    "owner_id": caller_id, 
                    "metadata": metadata, 
                    "token_metadata": token_metadata,
                    "minting_price": minting_price,
                    "perpetual_royalties": perpetual_royalties
                }).to_string().into_bytes(),
                NO_DEPOSIT,
                MAX_GAS
            );
    }

    pub fn is_subaccount_exist(&self, subaccount: String) -> bool {
        if self.subaccounts.contains(&subaccount) {
            return true;
        } else {
            return false;
        }
    }
}