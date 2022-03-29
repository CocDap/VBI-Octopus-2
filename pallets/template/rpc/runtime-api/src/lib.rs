#![cfg_attr(not(feature = "std"), no_std)]

use pallet_template::Store;
use pallet_template::Student;
use pallet_template::StudentAccount;
use codec::Codec;
sp_api::decl_runtime_apis! {
	pub trait SumStorageApi<Balance, Account>
	where Student<Balance, Account> : sp_api::Decode,


	{
        fn get_sum() -> u32;

		fn get_store() -> Store;

		fn get_student() -> Student<Balance, Account>;


	}
}