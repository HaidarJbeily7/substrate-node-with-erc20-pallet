use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn total_supply_should_be_zero_upon_creation() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(ERC20Token::total_supply(), 0);
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(ERC20Token::cause_error(RuntimeOrigin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
