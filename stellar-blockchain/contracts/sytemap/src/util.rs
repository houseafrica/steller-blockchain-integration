use crate::error::SytemapRegistryError;
use crate::storage::SytemapDataKeys;
use crate::types::{PropertyInfo, TokenMetadata};
use soroban_sdk::{Env, String, Vec};

//********** Storage Utils **********//

/// Retrieves the next available token ID from storage, and increments it.
pub fn get_next_token_id(env: &Env) -> Result<u64, SytemapRegistryError> {
    let next_token_id = env
        .storage()
        .instance()
        .get::<SytemapDataKeys, u64>(&SytemapDataKeys::NextTokenId)
        .unwrap_or(1);

    env.storage()
        .instance()
        .set(&SytemapDataKeys::NextTokenId, &(next_token_id + 1));

    Ok(next_token_id.try_into().unwrap())
}

/// Initializes the token ID tracker.
pub fn initialize_token_id_tracker(env: &Env) {
    // Set an initial token ID or retrieve from existing storage
    env.storage()
        .instance()
        .set(&SytemapDataKeys::TokenMetadata, &0);
}

/// Checks if a PropertyInfo exists for a given property verification number.
pub fn check_pvn_exists(env: &Env, pvn: u64) -> bool {
    let key = SytemapDataKeys::PvnToPropertyInfo;
    env.storage().instance().has(&key(pvn))
}

/// Checks if an NFT address to token ID mapping exists.
pub fn check_nft_address_exists(env: &Env, nft_address: String) -> bool {
    let key = SytemapDataKeys::NftAddressToTokenId;
    env.storage().instance().has(&key(nft_address))
}

/// Checks if token metadata exists.
pub fn token_metadata_exists(env: &Env) -> bool {
    let key = SytemapDataKeys::TokenMetadata;
    env.storage().instance().has(&key)
}

/// Checks if the AllProperties key exists.
pub fn all_properties_exists(env: &Env) -> bool {
    let key = SytemapDataKeys::AllProperties;
    env.storage().instance().has(&key)
}

/// Retrieves a `TokenMetadata` from storage.
pub fn get_token_metadata(env: &Env) -> Result<TokenMetadata, SytemapRegistryError> {
    env.storage()
        .instance()
        .get::<SytemapDataKeys, TokenMetadata>(&SytemapDataKeys::TokenMetadata)
        .ok_or(SytemapRegistryError::PropertyNotFound)
}

/// Retrieve all property verification numbers from storage.
pub fn get_all_property_verification_numbers(env: &Env) -> Vec<u64> {
    let key = SytemapDataKeys::AllProperties;
    env.storage()
        .instance()
        .get::<SytemapDataKeys, Vec<u64>>(&key)
        .unwrap_or_else(|| Vec::new(&env))
}

/// Saves a `PropertyInfo` to storage with a given property verification number.
pub fn mint_property_info(
    env: &Env,
    pvn: u64,
    property_info: &PropertyInfo,
) -> Result<(), SytemapRegistryError> {
    let key = SytemapDataKeys::PvnToPropertyInfo;
    env.storage().instance().set(&key(pvn), property_info);
    Ok(())
}

/// Saves a `TokenMetadata` to storage.
pub fn save_token_metadata(env: &Env, token_metadata: TokenMetadata) {
    let key = SytemapDataKeys::TokenMetadata;
    env.storage().instance().set(&key, &token_metadata);
}

/// Saves an NFT address to token ID mapping to storage.
pub fn save_nft_address_to_token_id(
    env: &Env,
    nft_address: String,
    token_id: u64,
) -> Result<(), SytemapRegistryError> {
    env.storage().instance().set(
        &SytemapDataKeys::NftAddressToTokenId(nft_address),
        &token_id,
    );
    Ok(())
}

/// Updates the list of all properties in storage.
pub fn update_all_properties(
    env: &Env,
    all_properties: Vec<u64>,
) -> Result<(), SytemapRegistryError> {
    env.storage()
        .instance()
        .set(&SytemapDataKeys::AllProperties, &all_properties);
    Ok(())
}

/// Save property info to storage.
pub fn save_property_info(
    env: &Env,
    pvn: u64,
    property_info: &PropertyInfo,
) -> Result<(), SytemapRegistryError> {
    env.storage()
        .instance()
        .set(&SytemapDataKeys::PvnToPropertyInfo(pvn), property_info);
    Ok(())
}

/// Save property verification number to token ID mapping.
pub fn save_pvn_to_token_id(
    env: &Env,
    pvn: u64,
    token_id: u64,
) -> Result<(), SytemapRegistryError> {
    env.storage().instance().set(
        &SytemapDataKeys::PropertyVerificationNoToTokenId(pvn),
        &token_id,
    );
    Ok(())
}

/// Retrieve property info from storage.
pub fn get_property_info(env: &Env, pvn: u64) -> Result<PropertyInfo, SytemapRegistryError> {
    env.storage()
        .instance()
        .get::<SytemapDataKeys, PropertyInfo>(&SytemapDataKeys::PvnToPropertyInfo(pvn))
        .ok_or(SytemapRegistryError::PropertyNotFound)
}

/// Retrieve the token ID associated with a given NFT address.
pub fn get_nft_address_to_token_id(
    env: &Env,
    nft_address: String,
) -> Result<u64, SytemapRegistryError> {
    env.storage()
        .instance()
        .get::<SytemapDataKeys, u64>(&SytemapDataKeys::NftAddressToTokenId(nft_address))
        .ok_or(SytemapRegistryError::PropertyNotFound)
}

/// Retrieves a `PropertyInfo` from storage by its property verification number.
pub fn get_property_info_by_pvn(
    env: &Env,
    property_verification_no: u64,
) -> Result<PropertyInfo, SytemapRegistryError> {
    env.storage()
        .instance()
        .get::<SytemapDataKeys, PropertyInfo>(&SytemapDataKeys::PvnToPropertyInfo(
            property_verification_no,
        ))
        .ok_or(SytemapRegistryError::PropertyNotFound)
}
