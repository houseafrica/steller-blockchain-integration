#![cfg(test)]
extern crate std;

use crate::error::*;
use crate::types::*;
use crate::util::*;
use crate::contract::{SytemapRegistry, SytemapRegistryClient};

// use super::DaoContract;
// use super::SytemapRegistryClient;

use soroban_sdk::{Address, Env, String};

// Initialize the environment and contract
fn get_client<'a>(e: &Env) -> SytemapRegistryClient<'a> {
    let contract_client = SytemapRegistryClient::new(e, &e.register_contract(None, SytemapRegistry {}));
    contract_client
}

#[test]
fn test_initialize_success() {
    let env: Env = Default::default();
    let contract_client = get_client(&env);

    let token_metadata = TokenMetadata {
        sytemap_name: String::from_str(&env, "Sytemap"),
        sytemap_symbol: String::from_str(&env, "SYM"),
        base_token_uri: String::from_str(&env, "http://metadata.url"),
    };

    // Call the `init` function
    let result = contract_client.initialize(&token_metadata);

    // assert!(result.is_ok());
    assert_eq!(token_metadata_exists(&env), true);
}

#[test]
fn test_initialize_already_initialized() {
    let env: Env = Default::default();

    let contract_client = get_client(&env);

    let token_metadata = TokenMetadata {
        sytemap_name: String::from_str(&env, "Sytemap"),
        sytemap_symbol: String::from_str(&env, "SYM"),
        base_token_uri: String::from_str(&env, "http://metadata.url"),
    };

    // Initialize once
    contract_client.initialize(&token_metadata).unwrap_err();

    // Attempt to initialize again
    let result = contract_client.initialize(&token_metadata);

    assert_eq!(result, Err(SytemapRegistryError::AlreadyInitialized));
}
#[test]
fn test_safe_mint_new_property_info_success() {
    let env: Env = Default::default();
    let contract_client = get_client(&env);
    // let buyer_wallet_id = Address::generate(&env);

    let token_metadata = TokenMetadata {
        sytemap_name: String::from_str(&env, "Sytemap"),
        sytemap_symbol: String::from_str(&env, "SYM"),
        base_token_uri: String::from_str(&env, "http://metadata.url"),
    };
    contract_client.init(&token_metadata).unwrap();

    let payload = PropertyInfoPayload {
        price_of_plot: 100_000,
        property_verification_no: 12345,
        buyer_wallet_id: Address::from_string(&env, "buyer_wallet_id"),
        plot_no: String::from_str(&env, "Plot123"),
        size_of_plot: String::from_str(&env, "10x10"),
        coordinate_of_plot: String::from_str(&env, "12.34, 56.78"),
        token_url: String::from_str(&env, "http://token.url"),
        estate_name: String::from_str(&env, "Estate XYZ"),
        plot_url: String::from_str(&env, "http://plot.url"),
        date_of_allocation: String::from_str(&env, "2024-08-01"),
        estate_company_name: String::from_str(&env, "Estate Co."),
        nft_address: String::from_str(&env, "nft_address"),
    };

    let result = contract_client.safe_mint_new_property_info(&env, payload);

    assert!(result.is_ok());
    assert_eq!(get_property_info_by_pvn(&env, 12345).is_ok(), true);
}

// #[test]
// fn test_safe_mint_new_property_info_already_minted() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address"),
//     };

//     contract.safe_mint_new_property_info(&env, payload.clone()).unwrap();

//     let result = contract.safe_mint_new_property_info(&env, payload);

//     assert_eq!(result, Err(SytemapRegistryError::AlreadyMinted));
// }

// #[test]
// fn test_safe_mint_new_property_info_nft_address_in_use() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address"),
//     };

//     contract.safe_mint_new_property_info(&env, payload.clone()).unwrap();

//     let payload2 = PropertyInfoPayload {
//         price_of_plot: 200_000,
//         property_verification_no: 67890,
//         buyer_wallet_id: Address::from("another_buyer_wallet_id"),
//         plot_no: String::from("Plot456"),
//         size_of_plot: String::from("20x20"),
//         coordinate_of_plot: String::from("98.76, 54.32"),
//         token_url: String::from("http://new_token.url"),
//         estate_name: String::from("Estate ABC"),
//         plot_url: String::from("http://new_plot.url"),
//         date_of_allocation: String::from("2024-08-02"),
//         estate_company_name: String::from("New Estate Co."),
//         nft_address: String::from("nft_address"), // Using the same nft_address
//     };

//     let result = contract.safe_mint_new_property_info(&env, payload2);

//     assert_eq!(result, Err(SytemapRegistryError::NftAddressAlreadyInUse));
// }

// #[test]
// fn test_change_property_price_by_owner_success() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address"),
//     };

//     contract.safe_mint_new_property_info(&env, payload).unwrap();

//     let new_price = 150_000;
//     let result = contract.change_property_price_by_owner(&env, 12345, new_price);

//     assert!(result.is_ok());
//     let property_info = get_property_info_by_pvn(&env, 12345).unwrap();
//     assert_eq!(property_info.price_of_plot, new_price);
// }

// #[test]
// fn test_change_property_price_by_owner_invalid_price() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address"),
//     };

//     contract.safe_mint_new_property_info(&env, payload).unwrap();

//     let result = contract.change_property_price_by_owner(&env, 12345, 0);

//     assert_eq!(result, Err(SytemapRegistryError::InvalidPrice));
// }

// #[test]
// fn test_get_property_info_details_by_pvn() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address"),
//     };

//     contract.safe_mint_new_property_info(&env, payload).unwrap();

//     let property_info = contract.get_property_info_details_by_pvn(&env, 12345).unwrap();

//     assert_eq!(property_info.property_verification_no, 12345);
//     assert_eq!(property_info.price_of_plot, 100_000);
// }

// #[test]
// fn test_get_property_info_details_by_nft_address() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address"),
//     };

//     contract.safe_mint_new_property_info(&env, payload).unwrap();

//     let property_info = contract.get_property_info_details_by_nft_address(&env, String::from("nft_address")).unwrap();

//     assert_eq!(property_info.property_verification_no, 12345);
//     assert_eq!(property_info.price_of_plot, 100_000);
// }

// #[test]
// fn test_get_total_number_of_property_owned_by_an_address_success() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload1 = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address1"),
//     };

//     let payload2 = PropertyInfoPayload {
//         price_of_plot: 200_000,
//         property_verification_no: 67890,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot456"),
//         size_of_plot: String::from("20x20"),
//         coordinate_of_plot: String::from("98.76, 54.32"),
//         token_url: String::from("http://new_token.url"),
//         estate_name: String::from("Estate ABC"),
//         plot_url: String::from("http://new_plot.url"),
//         date_of_allocation: String::from("2024-08-02"),
//         estate_company_name: String::from("New Estate Co."),
//         nft_address: String::from("nft_address2"),
//     };

//     contract.safe_mint_new_property_info(&env, payload1).unwrap();
//     contract.safe_mint_new_property_info(&env, payload2).unwrap();

//     let result = contract.get_total_number_of_property_owned_by_an_address(&env, Address::from("buyer_wallet_id")).unwrap();
//     assert_eq!(result, 2);
// }
// #[test]
// fn test_get_number_of_property_tokens_minted() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload1 = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address1"),
//     };

//     let payload2 = PropertyInfoPayload {
//         price_of_plot: 200_000,
//         property_verification_no: 67890,
//         buyer_wallet_id: Address::from("another_buyer_wallet_id"),
//         plot_no: String::from("Plot456"),
//         size_of_plot: String::from("20x20"),
//         coordinate_of_plot: String::from("98.76, 54.32"),
//         token_url: String::from("http://new_token.url"),
//         estate_name: String::from("Estate ABC"),
//         plot_url: String::from("http://new_plot.url"),
//         date_of_allocation: String::from("2024-08-02"),
//         estate_company_name: String::from("New Estate Co."),
//         nft_address: String::from("nft_address2"),
//     };

//     contract.safe_mint_new_property_info(&env, payload1).unwrap();
//     contract.safe_mint_new_property_info(&env, payload2).unwrap();

//     let result = contract.get_number_of_property_tokens_minted(&env).unwrap();
//     assert_eq!(result, 2);
// }

// #[test]
// fn test_get_property_verification_no_owner_success() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address"),
//     };

//     contract.safe_mint_new_property_info(&env, payload).unwrap();

//     let result = contract.get_property_verification_no_owner(&env, 12345).unwrap();
//     assert_eq!(result, Address::from("buyer_wallet_id"));
// }

// #[test]
// fn test_get_property_verification_no_owner_non_existent() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let result = contract.get_property_verification_no_owner(&env, 99999);
//     assert_eq!(result, Err(SytemapRegistryError::PropertyNotFound));
// }

// #[test]
// fn test_get_all_property_details_by_owner_success() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload1 = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address1"),
//     };

//     let payload2 = PropertyInfoPayload {
//         price_of_plot: 200_000,
//         property_verification_no: 67890,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot456"),
//         size_of_plot: String::from("20x20"),
//         coordinate_of_plot: String::from("98.76, 54.32"),
//         token_url: String::from("http://new_token.url"),
//         estate_name: String::from("Estate ABC"),
//         plot_url: String::from("http://new_plot.url"),
//         date_of_allocation: String::from("2024-08-02"),
//         estate_company_name: String::from("New Estate Co."),
//         nft_address: String::from("nft_address2"),
//     };

//     contract.safe_mint_new_property_info(&env, payload1).unwrap();
//     contract.safe_mint_new_property_info(&env, payload2).unwrap();

//     let properties = contract.get_all_property_details_by_owner(&env, Address::from("buyer_wallet_id")).unwrap();

//     assert_eq!(properties.len(), 2);
//     assert_eq!(properties[0].property_verification_no, 12345);
//     assert_eq!(properties[1].property_verification_no, 67890);
// }

// #[test]
// fn test_get_all_property_details_by_owner_no_properties() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let properties = contract.get_all_property_details_by_owner(&env, Address::from("non_existent_wallet_id")).unwrap();
//     assert_eq!(properties.len(), 0);
// }

// #[test]
// fn test_get_all_minted_property_details() {
//     let (env, contract) = get_client();

//     let token_metadata = TokenMetadata {
//         sytemap_name: String::from("Sytemap"),
//         sytemap_symbol: String::from("SYM"),
//         base_token_uri: String::from("http://metadata.url"),
//     };
//     contract.init(&env, token_metadata).unwrap();

//     let payload1 = PropertyInfoPayload {
//         price_of_plot: 100_000,
//         property_verification_no: 12345,
//         buyer_wallet_id: Address::from("buyer_wallet_id"),
//         plot_no: String::from("Plot123"),
//         size_of_plot: String::from("10x10"),
//         coordinate_of_plot: String::from("12.34, 56.78"),
//         token_url: String::from("http://token.url"),
//         estate_name: String::from("Estate XYZ"),
//         plot_url: String::from("http://plot.url"),
//         date_of_allocation: String::from("2024-08-01"),
//         estate_company_name: String::from("Estate Co."),
//         nft_address: String::from("nft_address1"),
//     };

//     let payload2 = PropertyInfoPayload {
//         price_of_plot: 200_000,
//         property_verification_no: 67890,
//         buyer_wallet_id: Address::from("another_buyer_wallet_id"),
//         plot_no: String::from("Plot456"),
//         size_of_plot: String::from("20x20"),
//         coordinate_of_plot: String::from("98.76, 54.32"),
//         token_url: String::from("http://new_token.url"),
//         estate_name: String::from("Estate ABC"),
//         plot_url: String::from("http://new_plot.url"),
//         date_of_allocation: String::from("2024-08-02"),
//         estate_company_name: String::from("New Estate Co."),
//         nft_address: String::from("nft_address2"),
//     };

//     contract.safe_mint_new_property_info(&env, payload1).unwrap();
//     contract.safe_mint_new_property_info(&env, payload2).unwrap();

//     let properties = contract.get_all_minted_property_details(&env).unwrap();
//     assert_eq!(properties.len(), 2);
//     assert_eq!(properties[0].property_verification_no, 12345);
//     assert_eq!(properties[1].property_verification_no, 67890);
// }
