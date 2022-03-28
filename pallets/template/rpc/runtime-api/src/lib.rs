#![cfg_attr(not(feature = "std"), no_std)]

use pallet_template::Store;
sp_api::decl_runtime_apis! {
	pub trait SumStorageApi
	{
        fn get_sum() -> u32;

		fn get_store() -> Store;
	}
}