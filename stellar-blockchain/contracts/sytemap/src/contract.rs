use crate::{
    error::SytemapRegistryError,
    events::SytemapRegistryEvents,
    types::{PropertyInfo, PropertyInfoPayload, TokenMetadata},
    util::*,
};

use soroban_sdk::{
    contract, contractimpl, contractmeta, log, panic_with_error, Address, Env, String, Vec,
};

contractmeta!(key = "stye", val = "Sytemap Coin for land tokenization");

/// Contract implementation for SytemapRegistry.
#[contract]
pub struct SytemapRegistry;

#[contractimpl]
impl SytemapRegistry {
    /// Initialize the contract with token metadata.
    pub fn initialize(
        env: Env,
        sytemap_name: String,
        sytemap_symbol: String,
        metadata_uri: String,
    ) {
        if token_metadata_exists(&env) {
            panic_with_error!(&env, SytemapRegistryError::AlreadyInitialized);
        }
        log!(&env, "token_metadata: {}", sytemap_symbol);

        initialize_token_id_tracker(&env); // Initialize token ID tracker
        save_token_metadata(
            &env,
            TokenMetadata {
                sytemap_name,
                sytemap_symbol,
                metadata_uri,
            },
        );
    }

    pub fn safe_mint_new_property_info(
        env: Env,
        payload: PropertyInfoPayload,
    ) -> Result<(), SytemapRegistryError> {
        let pvn = payload.property_verification_no;
        let nft_address = payload.nft_address.clone();

        if check_pvn_exists(&env, pvn) {
            panic_with_error!(&env, SytemapRegistryError::AlreadyMinted);
        }

        if check_nft_address_exists(&env, nft_address.clone()) {
            panic_with_error!(&env, SytemapRegistryError::NftAddressAlreadyInUse);
        }
        // Generate a new token ID
        let token_id = get_next_token_id(&env)?;
        log!(&env, "count: {}", token_id);

        // Capture the current timestamp from the ledger
        let timestamp = env.ledger().timestamp();

        // Create a new PropertyInfo with the timestamp included
        let property_info = PropertyInfo {
            price_of_plot: payload.price_of_plot,
            property_verification_no: pvn,
            buyer_wallet_id: payload.buyer_wallet_id,
            plot_no: payload.plot_no,
            size_of_plot: payload.size_of_plot,
            coordinate_of_plot: payload.coordinate_of_plot,
            token_url: payload.token_url,
            estate_name: payload.estate_name,
            plot_url: payload.plot_url,
            date_of_allocation: payload.date_of_allocation,
            estate_company_name: payload.estate_company_name,
            nft_address: payload.nft_address,
            timestamp, // Add timestamp to property_info
        };

        // // Save the property info, NFT address to token ID mapping, and property verification number to token ID mapping
        mint_property_info(&env, pvn, &property_info)?;
        save_nft_address_to_token_id(&env, nft_address, token_id.into())?;
        save_pvn_to_token_id(&env, pvn, token_id.into())?;

        // Track all properties
        let mut all_properties = get_all_property_verification_numbers(&env);
        all_properties.push_back(pvn);
        update_all_properties(&env, all_properties)?;

        // Emit event
        SytemapRegistryEvents::property_info_created(
            &env,
            property_info.plot_no.clone(),
            property_info.property_verification_no,
            token_id,
            timestamp,
            property_info.price_of_plot,
            property_info.buyer_wallet_id,
            property_info.token_url.clone(),
            property_info.estate_name.clone(),
            property_info.size_of_plot.clone(),
            property_info.plot_url.clone(),
            property_info.date_of_allocation.clone(),
            property_info.coordinate_of_plot.clone(),
            property_info.estate_company_name.clone(),
        );

        Ok(())
    }

    pub fn change_property_price_by_owner(
        env: Env,
        property_verification_no: u64,
        new_price: u64,
    ) -> Result<(), SytemapRegistryError> {
        if new_price == 0 {
            panic_with_error!(&env, SytemapRegistryError::InvalidPrice);
        }

        let mut property_info = get_property_info_by_pvn(&env, property_verification_no)?;

        property_info.price_of_plot = new_price;
        save_property_info(&env, property_verification_no, &property_info)?;

        // Emit event
        SytemapRegistryEvents::property_price_changed(
            &env,
            property_info.buyer_wallet_id,
            property_verification_no,
            new_price,
        );

        Ok(())
    }

    pub fn get_property_info_details_by_pvn(
        env: Env,
        property_verification_no: u64,
    ) -> Result<PropertyInfo, SytemapRegistryError> {
        get_property_info_by_pvn(&env, property_verification_no)
    }

    pub fn get_property_info_by_nft_address(
        env: Env,
        nft_address: String,
    ) -> Result<PropertyInfo, SytemapRegistryError> {
        let token_id = get_nft_address_to_token_id(&env, nft_address)?;
        get_property_info(&env, token_id)
    }

    pub fn get_no_of_property_by_address(
        env: Env,
        owner_address: Address,
    ) -> Result<u64, SytemapRegistryError> {
        let all_properties = get_all_property_verification_numbers(&env);

        let count = all_properties
            .iter()
            .filter_map(|pvn| match get_property_info_by_pvn(&env, pvn) {
                Ok(property_info) => {
                    if property_info.buyer_wallet_id == owner_address {
                        Some(())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            .count() as u64;

        Ok(count)
    }

    pub fn get_number_of_property_minted(env: Env) -> Result<u64, SytemapRegistryError> {
        let all_properties = get_all_property_verification_numbers(&env);
        Ok(all_properties.len() as u64)
    }

    pub fn get_owner_of_pvn(
        env: Env,
        property_verification_no: u64,
    ) -> Result<Address, SytemapRegistryError> {
        let property_info = get_property_info_by_pvn(&env, property_verification_no)?;
        Ok(property_info.buyer_wallet_id)
    }

    pub fn get_all_property_by_owner(
        env: Env,
        owner_address: Address,
    ) -> Result<soroban_sdk::Vec<PropertyInfo>, SytemapRegistryError> {
        // Retrieve all property verification numbers using both key and value types
        let all_properties = get_all_property_verification_numbers(&env);

        // Initialize a soroban_sdk::Vec to store the filtered property details
        let mut properties = soroban_sdk::Vec::new(&env);

        for pvn in all_properties.iter() {
            // Retrieve the property information using both key and value types
            match get_property_info(&env, pvn) {
                Ok(property_info) => {
                    if property_info.buyer_wallet_id == owner_address {
                        properties.push_back(property_info);
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(properties)
    }

    pub fn get_all_minted_property_details(
        env: Env,
    ) -> Result<soroban_sdk::Vec<PropertyInfo>, SytemapRegistryError> {
        // Retrieve all property verification numbers using both key and value types
        let all_properties = get_all_property_verification_numbers(&env);

        // Initialize a soroban_sdk::Vec to store the property details
        let mut properties = Vec::new(&env);

        // Iterate over all property verification numbers and retrieve property details
        for pvn in all_properties {
            if let Ok(property_info) = get_property_info(&env, pvn) {
                properties.push_back(property_info);
            }
        }

        Ok(properties)
    }
}
