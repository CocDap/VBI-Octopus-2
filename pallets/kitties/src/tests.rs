use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_working_create_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));

	});
}
