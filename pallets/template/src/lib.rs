#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Saturating;
use weights::WeightInfo;
use frame_support::traits::Currency;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Encode, Decode, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Store {
	pub value: u64
}

#[derive(Clone, Encode, Decode, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(bound(serialize = "Balance: std::fmt::Display")))]
#[cfg_attr(feature = "std", serde(bound(deserialize = "Balance: std::str::FromStr")))]
pub struct Student<Balance> {
	#[cfg_attr(feature = "std", serde(with = "serde_balance"))]
	amount: Balance,

	age: u64,
}

#[cfg(feature = "std")]
mod serde_balance {
	use serde::{Deserialize, Deserializer, Serializer};

	pub fn serialize<S: Serializer, T: std::fmt::Display>(
		t: &T,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&t.to_string())
	}

	pub fn deserialize<'de, D: Deserializer<'de>, T: std::str::FromStr>(
		deserializer: D,
	) -> Result<T, D::Error> {
		let s = String::deserialize(deserializer)?;
		s.parse::<T>().map_err(|_| serde::de::Error::custom("Parse from string failed"))
	}
}


type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {

	pub use super::*;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		type WeightInfo : WeightInfo;
		type Currency : Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;


	#[pallet::storage]
	#[pallet::getter(fn something2)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something2<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn set_something1(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn set_something2(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something2<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}

//helper function for pallet template
impl<T: Config> Pallet<T>{

	pub fn update_storage(value : u32) -> DispatchResult{

		Something::<T>::put(value);
		Ok(())
	}

	pub fn sum_storage() -> u32{
		Something::<T>::get().unwrap() + Something2::<T>::get().unwrap()
	}

	pub fn get_store() -> Store{

		let store = Store { value : 10u64};
		store
	}


	pub fn get_student() -> Student<BalanceOf<T>>{
		let amount: BalanceOf<T> = 10u32.into(); 
		let student = Student{ amount: amount, age: 18u64};
		student
	}
}

pub trait DoSome {
	fn increase_value(value: u32) -> u32;
}


impl<T:Config> DoSome for Pallet<T>{

	fn increase_value(value : u32) -> u32 {
		value.saturating_add(5)
	}
}


