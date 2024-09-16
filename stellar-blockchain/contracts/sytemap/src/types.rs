use soroban_sdk::{contracttype, Address, String};

/// Object for storing token meta data
#[contracttype]
pub struct TokenMetadata {
    pub sytemap_name: String,
    pub sytemap_symbol: String,

    /// The Asset Metadata is a URI where the metadata is hosted, this returns an String
    pub metadata_uri: String,
}

/// Object for storing property info data
#[derive(Debug, Clone)]
#[contracttype]
pub struct PropertyInfo {
    pub price_of_plot: u64,
    pub property_verification_no: u64,
    pub timestamp: u64,
    pub buyer_wallet_id: Address,
    pub plot_no: String,
    pub size_of_plot: String,
    pub coordinate_of_plot: String,
    pub token_url: String,
    pub estate_name: String,
    pub plot_url: String,
    pub date_of_allocation: String,
    pub estate_company_name: String,
    pub nft_address: String,
}

/// Object for storing property info data
#[derive(Debug, Clone)]
#[contracttype]
pub struct PropertyInfoPayload {
    pub price_of_plot: u64,
    pub property_verification_no: u64,
    pub buyer_wallet_id: Address,
    pub plot_no: String,
    pub size_of_plot: String,
    pub coordinate_of_plot: String,
    pub token_url: String,
    pub estate_name: String,
    pub plot_url: String,
    pub date_of_allocation: String,
    pub estate_company_name: String,
    pub nft_address: String,
}
