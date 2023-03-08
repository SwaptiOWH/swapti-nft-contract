use near_sdk::{
    env, serde_json::json
};

use crate::*;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Contract {
    pub fn create_profile(&mut self, email: String, bio: String) -> String {
        let accountid = env::signer_account_id().clone();
        // Verificar que el perfil no exista
        let p = self.profiles.get(&accountid.clone());
        if p.is_some() {
            env::panic_str("Ya existe un perfil para esta cuenta");
        }
        let new_profile = Profile {
            email : email,
            bio : bio,
        };
        self.profiles.insert(accountid.clone(),new_profile);

        ext_nft::mint_swapti(
            accountid.clone(),
            "1000000000000000000000000".to_string(),
            "dev-1678227171447-27568759288636".to_string().parse::<AccountId>().unwrap(),
            NO_DEPOSIT,
            MIN_GAS_FOR_NFT_TRANSFER_CALL
        );

        return "Perfil creado con éxito".to_string();
    }

    pub fn get_profile(&self, account_id: AccountId) -> Profile {
        // Verificar que el perfil no exista
        let p = self.profiles.get(&account_id.clone());
        if p.is_none() {
            env::panic_str("No se encontró información de perfil");
        }

        let info = p.unwrap();

        let profile_info = Profile {
            email : info.email.to_string(),
            bio : info.bio.to_string()
        };

        return profile_info;
    }
}