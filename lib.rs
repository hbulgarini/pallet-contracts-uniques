#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;

use ink_lang as ink;
pub use xcm::{VersionedMultiAsset, VersionedMultiLocation, VersionedResponse, VersionedXcm};

type DefaultAccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
type DefaultBalance = <ink_env::DefaultEnvironment as Environment>::Balance;

#[ink::chain_extension]
pub trait Psp02Extension {
    type ErrorCode = Psp02Error;

    #[ink(extension = 0x162d)]
    fn get_owner(asset_id: u32, collection_id: u32) -> Result<DefaultAccountId>;

    // PSP22 transfer
    #[ink(extension = 0xdb20)]
    fn transfer(asset_id: u32, dest: DefaultAccountId, collection_id: u32)
    -> Result<()>;
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Psp02Error {
    TotalSupplyFailed,
}

pub type Result<T> = core::result::Result<T, Psp02Error>;

impl From<scale::Error> for Psp02Error {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for Psp02Error {
    fn from_status_code(status_code: u32) -> core::result::Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::TotalSupplyFailed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = Psp02Extension;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod psp02_ext {
    use crate::DefaultAccountId;

    use super::{
        Result
    };

    /// A chain extension which implements the PSP-22 fungible token standard.
    /// For more details see <https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md>
    #[ink(storage)]
    #[derive(Default)]
    pub struct Psp02Extension {}

    impl Psp02Extension {
        /// Creates a new instance of this contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        // PSP22 Metadata interfaces

        /// Returns the token name of the specified asset.
        #[ink(message)]
        pub fn get_owner(&self, asset_id: u32, collection_id: u32) -> Result<DefaultAccountId> {
            self.env().extension().get_owner(asset_id, collection_id)
        }
        // PSP22 transfer

        /// Transfers `value` amount of specified asset from the caller's account to the
        /// account `to`.
        #[ink(message)]
        pub fn transfer_nft(
            &mut self,
            asset_id: u32, dest: DefaultAccountId, collection_id: u32
        ) -> Result<()> {
            self.env().extension().transfer(asset_id, dest, collection_id)
        }
    }
}
