#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use scale_info::prelude::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, Blake2_128Concat};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type TokenName: Get<Vec<u8>>;
		#[pallet::constant]
		type TokenSymbol: Get<Vec<u8>>;
		#[pallet::constant]
		type TokenDecimals: Get<u8>;
	}

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub type TotalSupply<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn balances)]
	pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn allowances)]
	pub type Allowances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		u64,
		ValueQuery,
	>;

	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Transfer { from: T::AccountId, to: T::AccountId, amount: u64 },
		Approval { owner: T::AccountId, spender: T::AccountId, amount: u64 },
		Mint { account: T::AccountId, amount: u64 },
		Burn { account: T::AccountId, amount: u64 },
	}

	#[pallet::error]
	pub enum Error<T> {
		ERC20InvalidSender,
		ERC20InvalidReceiver,
		ERC20InvalidApprover,
		ERC20InvalidSpender,
		ERC20InsufficientAllowance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: u64) -> DispatchResult {
			let owner = ensure_signed(origin)?;

			Ok(())
		}

		// #[pallet::call_index(1)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn transferFrom(
		// 	origin: OriginFor<T>,
		// 	from: T::AccountId,
		// 	to: T::AccountId,
		// 	amount: u64,
		// ) -> DispatchResult {
		// 	let owner = ensure_signed(origin)?;
		// 	ensure!()
		// 	Ok(())
		// }

		// #[pallet::call_index(2)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn approve(origin: OriginFor<T>, to: T::AccountId, amount: u64) -> DispatchResult {
		// 	let owner = ensure_signed(origin)?;

		// 	Ok(())
		// }
	}

	// fn _transfer<T: Config>(from: T::AccountId, to: T::AccountId, value: u64) -> DispatchResult {
	// 	if from == T::Root::root() {
	// 		return Ok(())
	// 	}
	// 	Err(Error::ERC20InvalidSender.into());
	// }
	fn _burn() {}

	fn _mint() {}

	fn _approve() {}

	fn _spendAllownace() {}
}
