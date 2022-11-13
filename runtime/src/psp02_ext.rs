use codec::{
    Decode,
    Encode,
    MaxEncodedLen,
};
use frame_support::{
    DefaultNoBound,
    dispatch::RawOrigin,
    log::{
        error,
        trace,
    },
    pallet_prelude::*,
    traits::tokens::nonfungibles::{
        Inspect,
        Transfer,
    },
};
pub use sp_core::{
	crypto::{
		Wraps,
	},
};
use pallet_uniques::{
    self,
    WeightInfo,
};
use pallet_contracts::chain_extension::{
    ChainExtension,
    Environment,
    Ext,
    InitState,
    RetVal,
    SysConfig,
    UncheckedFrom,
    RegisteredChainExtension
};
use sp_runtime::{
    traits::{
        Saturating,
        StaticLookup,
        Zero,
    },
    DispatchError,
};

use super::*;

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp02PriceOf<ItemId, AccountId> {
    item_id: ItemId,
    owner: AccountId,
}

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Item<ItemId, CollectionId> {
    item_id: ItemId,
    collection_id: CollectionId,
}

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp02TransferInput<ItemId, CollectionId, AccountId> {
    collection_id: CollectionId,
    item_id: ItemId,
    dest: AccountId
}

#[derive(Default)]
pub struct Psp02Extension {

}

fn convert_err(err_msg: &'static str) -> impl FnOnce(DispatchError) -> DispatchError {
    move |err| {
        trace!(
            target: "runtime",
            "PSP02 Transfer failed:{:?}",
            err
        );
        DispatchError::Other(err_msg)
    }
}

/// We're using enums for function IDs because contrary to raw u16 it enables
/// exhaustive matching, which results in cleaner code.
enum FuncId {
    QueryOwner,
    Transfer
}

#[derive(Debug)]
enum Query {
    Owner
}

impl TryFrom<u16> for FuncId {
    type Error = DispatchError;

    fn try_from(func_id: u16) -> Result<Self, Self::Error> {
        let id = match func_id {
            // Note: We use the first two bytes of PSP22 interface selectors as function IDs,
            // While we can use anything here, it makes sense from a convention perspective.
            0x162d => Self::QueryOwner,
            0xdb20 => Self::Transfer,
            _ => {
                error!("Called an unregistered `func_id`: {:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"))
            }
        };

        Ok(id)
    }
} 

/* fn query<T, E>(
    func_id: Query,
    env: Environment<E, InitState>,
) -> Result<(), DispatchError>
where
    T: pallet_uniques::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8; 32]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();
    let result = match func_id {
        Query::Owner => {
            let input: Psp02TransferInput<T::ItemId, T::CollectionId, T::AccountId> = env.read_as_unbounded()?;
            let Psp02TransferInput {collection_id,  item_id, dest: _} = input;
            <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::owner(&collection_id,&item_id)
        }
    }
    .encode();
    trace!(
        target: "runtime",
        "[ChainExtension] PSP22::{:?}",
        func_id
    );
    env.write(&result, false, None)
        .map_err(convert_err("ChainExtension failed to call PSP22 query"))
}

fn transfer<T, E>(env: Environment<E, InitState>) -> Result<(), DispatchError>
where
    T: pallet_uniques::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8; 32]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();
    let base_weight = <T as pallet_uniques::Config>::WeightInfo::transfer();
    // debug_message weight is a good approximation of the additional overhead of going from
    // contract layer to substrate layer.
    let overhead = Weight::from_ref_time(
        <T as pallet_contracts::Config>::Schedule::get()
            .host_fn_weights
            .debug_message,
    );
    let charged_weight = env.charge_weight(base_weight.saturating_add(overhead))?;
    trace!(
        target: "runtime",
        "[ChainExtension]|call|transfer / charge_weight:{:?}",
        charged_weight
    );

    let input: Psp02TransferInput<T::ItemId, T::CollectionId, T::AccountId> = env.read_as()?;
    let Psp02TransferInput {collection_id,  item_id, dest} = input;
    let sender = env.ext().caller();

    <pallet_uniques::Pallet<T> as Transfer<T::AccountId>>::transfer(
        &collection_id,
        &item_id,
        &dest
    )
    .map_err(convert_err("ChainExtension failed to call transfer"))?;
    trace!(
        target: "runtime",
        "[ChainExtension]|call|transfer"
    );

    Ok(())
} */

impl<T: Config> ChainExtension<T> for Psp02Extension
where
    T: pallet_uniques::Config + pallet_contracts::Config,
	<T as SysConfig>::AccountId: AsRef<[u8; 32]>
{
	fn call<E>(&mut self, mut env: Environment<E, InitState>) -> Result<RetVal,DispatchError>
	where
		E: Ext<T = T>,
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
        let func_id = FuncId::try_from(env.func_id())?;
        match func_id {
            FuncId::QueryOwner => {
                let mut env = env.buf_in_buf_out();
                let input: Psp02TransferInput<T::ItemId, T::CollectionId, T::AccountId> = env.read_as()?;
                let Psp02TransferInput {collection_id,  item_id, dest: _} = input;
                let result = <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::owner(&collection_id,&item_id);
/*                 env.write(&result.encode(), false, None)
                .map_err(convert_err("ChainExtension failed to call PSP22 query")) */
            },
            FuncId::Transfer => {
                let mut env = env.buf_in_buf_out();
                let base_weight = <T as pallet_uniques::Config>::WeightInfo::transfer();
                // debug_message weight is a good approximation of the additional overhead of going from
                // contract layer to substrate layer.
                let overhead = Weight::from_ref_time(
                    <T as pallet_contracts::Config>::Schedule::get()
                        .host_fn_weights
                        .debug_message,
                );
                let charged_weight = env.charge_weight(base_weight.saturating_add(overhead))?;
                trace!(
                    target: "runtime",
                    "[ChainExtension]|call|transfer / charge_weight:{:?}",
                    charged_weight
                );
            
                let input: Psp02TransferInput<T::ItemId, T::CollectionId, T::AccountId> = env.read_as()?;
                let Psp02TransferInput {collection_id,  item_id, dest} = input;
                let sender = env.ext().caller();
            
                <pallet_uniques::Pallet<T> as Transfer<T::AccountId>>::transfer(
                    &collection_id,
                    &item_id,
                    &dest
                )?;
            }
        }
        Ok(RetVal::Converging(0))
	}
}

impl<T: Config> RegisteredChainExtension<T> for Psp02Extension
where
	<T as SysConfig>::AccountId: AsRef<[u8; 32]>
{
	const ID: u16 = 2;
}