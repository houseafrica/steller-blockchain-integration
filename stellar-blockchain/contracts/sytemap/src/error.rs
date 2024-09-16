use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SytemapRegistryError {
    AlreadyMinted = 1,
    PropertyAlreadyExists = 2,
    InvalidPrice = 3,
    PropertyNotFound = 4,
    NftAddressAlreadyInUse = 5,
    AlreadyInitialized = 6,
}
