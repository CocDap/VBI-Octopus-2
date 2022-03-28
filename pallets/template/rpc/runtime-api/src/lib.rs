#![cfg_attr(not(feature = "std"), no_std)]

use pallet_template::Store;
use pallet_template::Student;
use codec::Codec;
sp_api::decl_runtime_apis! {
	pub trait SumStorageApi<Balance> where Balance:Codec, Student<Balance>: sp_api::Decode,
	{
        fn get_sum() -> u32;

		fn get_store() -> Store;

		fn get_student() -> Student<Balance>;
	}
}