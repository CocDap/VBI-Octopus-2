//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as KittiesModule;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use frame_benchmarking::account;
use pallet_balances::Pallet as Balances;
use frame_support::traits::OnUnbalanced;
use frame_support::traits::Currency;

/// Grab a funded user.
pub fn create_funded_user<T: Config>(
	string: &'static str,
	n: u32,
	balance_factor: u32,
) -> T::AccountId {
	let user = account(string, n, 1);
	let balance = T::Currency::minimum_balance() * balance_factor.into();
	let _ = T::Currency::make_free_balance_be(&user, balance);
	user
}

benchmarks! {
	transfer{
		let s in 0 .. 100;
		//let caller: T::AccountId = whitelisted_caller();
		let caller = create_funded_user::<T>("caller", 1, 1);
		let to: T::AccountId = account("to", 2u32, 2u32);
		let _ = KittiesModule::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		let kitty_hashes = KittiesModule::<T>::kitties_owned(caller.clone());
		//Balances::<T>::set_balance( RawOrigin::Root.into(), to.clone(), s.into(), 0);
		//let _ = <Balances<_> as Currency<_>>::make_free_balance_be(&caller, 10u128);
	} : _(RawOrigin::Signed(caller), to, kitty_hashes[0])

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
