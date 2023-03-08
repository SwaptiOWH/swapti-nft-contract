use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue, Gas
};
type U128String = U128;

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::royalty::*;
pub use crate::events::*;
pub use crate::profile::*;
pub use crate::xcc::*;

mod internal;
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;
mod profile;
mod xcc;


/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";
pub const ICON: &str = "data:image/jpeg;base64,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";
const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    TokensToMintCounter,
}
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum RequestStatus {
    Open,
    InProgress,
    Complete,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Review {
    rating: u8,
    comment: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Profile {
    email: String,
    bio: String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TimesHelped {
    number: u64,
    bronze: bool,
    silver: bool,
    gold: bool
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Request {
    request_id: u64,
    owner_id: AccountId,
    description: String,
    helper: Option<AccountId>,
    status: RequestStatus,
    review_owner: Option<Review>,
    review_helper: Option<Review>
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    pub requests_per_owner: LookupMap<AccountId, UnorderedSet<u64>>,
    pub requests: UnorderedMap<u64, Request>,
    pub profiles: HashMap<AccountId, Profile>,
    pub profile_times_helped: HashMap<AccountId, TimesHelped>

}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "SWAPTI NFT".to_string(),
                symbol: "ST".to_string(),
                icon: Some(ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            requests_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            requests: UnorderedMap::new(b"m"),
            profiles: HashMap::new(),
            profile_times_helped: HashMap::new()
        };

        //return the Contract object
        this
    }

    pub fn create_request(&mut self, description: String) -> String {
        let request_id = self.requests.len() as u64;

        let request = Request {
            request_id: request_id.clone(),
            owner_id: env::signer_account_id().clone(),
            description,
            helper: None,
            status: RequestStatus::Open,
            review_owner: None,
            review_helper: None
        };

        self.requests.insert(&request_id, &request);
        self.internal_add_request_to_owner(&env::signer_account_id(), &request_id);
        return "Solicitud creada con éxito".to_string();
    }

    pub fn request_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Request> {
        let request_for_owner_set = self.requests_per_owner.get(&account_id);
        let request = if let Some(request_for_owner_set) = request_for_owner_set {
            request_for_owner_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        request.iter()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|request_id| self.get_request(request_id.clone()).unwrap())
            .collect()
    }

    pub fn all_requests(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Request> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        self.requests.keys()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|request_id| self.get_request(request_id.clone()).unwrap())
            .collect()
    }

    pub fn get_request(&self, request_id: u64) -> Option<Request> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(request) = self.requests.get(&request_id) {
            //we'll get the metadata for that token
            let request_data = self.requests.get(&request_id).unwrap();
            //we return the JsonToken (wrapped by Some since we return an option)
            Some(Request {
                request_id: request_id,
                owner_id: request_data.owner_id,
                description: request_data.description,
                helper: request_data.helper,
                status: request_data.status,
                review_owner: request_data.review_owner,
                review_helper: request_data.review_helper
            })
        } else {
            None
        }
    }


    pub fn get_number_swaps(&self, accountid: AccountId) -> TimesHelped{
        let p = self.profile_times_helped.get(&accountid.clone());
        if p.is_none() {
            let profile_info = TimesHelped {
                number: 0,
                bronze: false,
                silver: false,
                gold: false,
            };
            return profile_info;
        }

        let info = p.unwrap();
        let profile_info = TimesHelped {
            number: info.number,
            bronze: info.bronze,
            silver: info.silver,
            gold: info.gold,
        };
        return profile_info;
    }

    pub fn attend_request(&mut self, request_id: u64) -> Option<Request> {
        if let Some(request) = self.requests.get(&request_id) {
            let request_data = self.requests.get(&request_id).unwrap();

            if request_data.owner_id == env::signer_account_id().clone() {
                env::panic_str("No puedes atender tu propia solicitud");
            }

            if request_data.status == RequestStatus::InProgress || request_data.status == RequestStatus::Complete {
                env::panic_str("La solicitud ya fué atendida");
            }

            let new_request = Request {
                request_id: request_id,
                owner_id: request_data.owner_id,
                description: request_data.description,
                helper: Some(env::signer_account_id().clone()),
                status: RequestStatus::InProgress,
                review_owner: request_data.review_owner,
                review_helper: request_data.review_helper
            };

            self.requests.insert(&request_id, &new_request);

            return Some(new_request);


        } else {
            env::panic_str("No existe la solicitud a atender");
        }
    }

    pub fn finish_request(&mut self, request_id: u64) -> Option<Request> {
        if let Some(request) = self.requests.get(&request_id) {
            let request_data = self.requests.get(&request_id).unwrap();

            if request_data.owner_id != env::signer_account_id().clone() {
                env::panic_str("Solo el creador de la solicitud puede finalizarla");
            }

            if request_data.status == RequestStatus::Open || request_data.status == RequestStatus::Complete {
                env::panic_str("La solicitud aún no es atendida o ya finalizó");
            }

            let new_request = Request {
                request_id: request_id,
                owner_id: request_data.owner_id,
                description: request_data.description,
                helper: request_data.helper.clone(),
                status: RequestStatus::Complete,
                review_owner: request_data.review_owner,
                review_helper: request_data.review_helper
            };

            self.requests.insert(&request_id, &new_request);

            // Hacer el envió del swapti token
            ext_nft::ft_transfer(
                request_data.helper.unwrap(),
                "1000000000000000000000000".to_string().clone(),
                "dev-1678227171447-27568759288636".to_string().parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                MIN_GAS_FOR_NFT_TRANSFER_CALL
            );

            return Some(new_request);


        } else {
            env::panic_str("No existe la solicitud a atender");
        }
    }
    
}

#[cfg(test)]
mod tests;