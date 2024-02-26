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
		ERC20InsufficientAllowance,
		ERC20InsufficientBalance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let sender_balance = Balances::<T>::get(&sender);
			let receiver_balance = Balances::<T>::get(&to);
			ensure!(sender_balance >= amount, Error::<T>::ERC20InsufficientBalance);
			<Balances<T>>::set(&sender, sender_balance - amount);
			<Balances<T>>::set(&to, receiver_balance + amount);
			Self::deposit_event(Event::Transfer { from: sender.clone(), to, amount });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn transfer_from(
			origin: OriginFor<T>,
			from: T::AccountId,
			to: T::AccountId,
			amount: u64,
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;
			let sender_balance = Balances::<T>::get(&from);
			let receiver_balance = Balances::<T>::get(&to);
			let allownce = <Allowances<T>>::get(&from, &spender);
			ensure!(allownce >= amount, Error::<T>::ERC20InsufficientAllowance);
			ensure!(sender_balance >= amount, Error::<T>::ERC20InsufficientBalance);
			<Allowances<T>>::set(&from, &spender, allownce - amount);
			<Balances<T>>::set(&from, sender_balance - amount);
			<Balances<T>>::set(&to, receiver_balance + amount);
			Self::deposit_event(Event::Transfer { from, to, amount });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn approve(origin: OriginFor<T>, spender: T::AccountId, amount: u64) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			<Allowances<T>>::set(&owner, &spender, amount);
			Self::deposit_event(Event::Approval { owner, spender, amount });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn mint(origin: OriginFor<T>, amount: u64) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let total_supply = <TotalSupply<T>>::get().unwrap();
			<TotalSupply<T>>::put(total_supply + amount);
			<Balances<T>>::set(&owner, <Balances<T>>::get(&owner) + amount);
			Self::deposit_event(Event::Mint { account: owner.clone(), amount });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn burn(origin: OriginFor<T>, amount: u64) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let owner_balance = <Balances<T>>::get(&owner);
			ensure!(owner_balance >= amount, Error::<T>::ERC20InsufficientBalance);
			let total_supply = <TotalSupply<T>>::get().unwrap();
			<TotalSupply<T>>::put(total_supply - amount);
			<Balances<T>>::set(&owner, owner_balance - amount);
			Self::deposit_event(Event::Burn { account: owner.clone(), amount });
			Ok(())
		}
	}
}
