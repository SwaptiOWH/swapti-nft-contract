use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json,from_str};
use near_sdk::{env,ext_contract, Balance,Gas, near_bindgen, AccountId, PromiseOrValue, PromiseResult, PanicOnDefault, log, BorshStorageKey, require};
use near_sdk::json_types::{U128, U64};
use near_sdk::Promise;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};

pub const BRONCE: &str = "QmT6tLkZMbqvDKXF8pCMMew8fFs17c2z98XGrmvBb2AJ9W";
pub const PLATA: &str = "QmZ3at5KDTa8a1zhs3qAgkMpQ48gbWoN53ZLJw4A88WyK2";
pub const ORO: &str = "Qmd1U8jL4eEn8gsBvQfhXmn2oaNfF9GwCWxQ3UEkccdZV9";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtraData {
    init: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NewTokenMetadata {
    title: String, 
    description: String, 
    media: String, 
    extra: String
}

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn mint_bronce( &mut self) -> String {
        let receiver_id = env::predecessor_account_id();

        let profile_times_helped = self.profile_times_helped.get(&receiver_id);
        
        if profile_times_helped.is_none() {
            env::panic_str("Aún no has ayudado a ninguna persona");
        }

        let info = profile_times_helped.unwrap();

        if info.number < 10 {
            env::panic_str("Necesitas al menos ayudar a 10 personas");
        }

        if info.bronze {
            env::panic_str("Ya minaste este token");
        }

        let deposit = env::attached_deposit();
        let mut new_token = TokenMetadata {
            title:  Some("Insignia de Bronce".to_string()), 
            description:  Some("Este NFT conmemora que haz ayudado a 10 personas".to_string()),
            media:  Some("".to_string()),
            expires_at: None,
            starts_at: None,
            copies: None,
            extra: None,
            issued_at: None,
            media_hash: None,
            reference: None,
            reference_hash: None,
            updated_at: None
        };

        let initial_storage_usage = env::storage_usage();

        new_token.media = Some(BRONCE.to_string());
        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &new_token);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);


        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    
        return "El token fué minado con éxito".to_string();
    }

    #[payable]
    pub fn mint_plata( &mut self) -> String {
        // Agregar validación para verificar que ya ayudaste a minimo 25 personas

        let receiver_id = env::predecessor_account_id();

        let profile_times_helped = self.profile_times_helped.get(&receiver_id);
        
        if profile_times_helped.is_none() {
            env::panic_str("Aún no has ayudado a ninguna persona");
        }

        let info = profile_times_helped.unwrap();

        if info.number < 25 {
            env::panic_str("Necesitas al menos ayudar a 25 personas");
        }

        if info.silver {
            env::panic_str("Ya minaste este token");
        }

        let deposit = env::attached_deposit();
        let mut new_token = TokenMetadata {
            title:  Some("Insignia de Plata".to_string()), 
            description:  Some("Este NFT conmemora que haz ayudado a 25 personas".to_string()),
            media:  Some("".to_string()),
            expires_at: None,
            starts_at: None,
            copies: None,
            extra: None,
            issued_at: None,
            media_hash: None,
            reference: None,
            reference_hash: None,
            updated_at: None
        };

        let initial_storage_usage = env::storage_usage();

        new_token.media = Some(PLATA.to_string());
        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &new_token);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);


        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    
        return "El token fué minado con éxito".to_string();
    }

    #[payable]
    pub fn mint_oro( &mut self) -> String {
        // Agregar validación para verificar que ya ayudaste a minimo 50 personas
        let receiver_id = env::predecessor_account_id();

        let profile_times_helped = self.profile_times_helped.get(&receiver_id);
        
        if profile_times_helped.is_none() {
            env::panic_str("Aún no has ayudado a ninguna persona");
        }

        let info = profile_times_helped.unwrap();

        if info.number < 50 {
            env::panic_str("Necesitas al menos ayudar a 50 personas");
        }

        if info.gold {
            env::panic_str("Ya minaste este token");
        }

        let deposit = env::attached_deposit();
        let mut new_token = TokenMetadata {
            title:  Some("Insignia de ORO".to_string()), 
            description:  Some("Este NFT conmemora que haz ayudado a 50 personas".to_string()),
            media:  Some("".to_string()),
            expires_at: None,
            starts_at: None,
            copies: None,
            extra: None,
            issued_at: None,
            media_hash: None,
            reference: None,
            reference_hash: None,
            updated_at: None
        };

        let initial_storage_usage = env::storage_usage();

        new_token.media = Some(ORO.to_string());
        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &new_token);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);


        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    
        return "El token fué minado con éxito".to_string();
    }

}