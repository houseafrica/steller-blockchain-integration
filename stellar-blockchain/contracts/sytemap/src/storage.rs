use soroban_sdk::{contracttype, String};

//********** Storage Keys **********//

#[contracttype]
pub enum SytemapDataKeys {
    // A map of token id to PropertyVerificationNoToTokenId
    PropertyVerificationNoToTokenId(u64),
    NftAddressToTokenId(String), // For nftAddress to token ID
    // A map property verification number to PropertyInfo
    PvnToPropertyInfo(u64),
    SystemName,
    SystemSymbol,
    PropertyInfo(u64),
    TokenMetadata,
    AllProperties, // New key to track all properties
    NextTokenId, // Key to track the next token ID

}
