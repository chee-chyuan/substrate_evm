use std::str::FromStr;

use frame_support::sp_runtime::app_crypto::sp_core::{H160, U256};

use super::contracts::simple_storage::STORAGE_BYTECODE;
use crate::mock::{new_test_ext, EVM};

#[test]
fn test_account_init() {
	let (pairs, mut ext) = new_test_ext(1);
    let alice_addr = &pairs[0].address;
	ext.execute_with(|| {
		let is_account_empty = EVM::is_account_empty(&alice_addr);
		assert_eq!(is_account_empty, false);

		let basic = EVM::account_basic(&alice_addr);
		println!("{:?}", basic);
		println!("{:?}", U256::max_value().low_u128());
		println!("{:?}", U256::max_value());
		assert_eq!(basic.0.balance, U256::from(U256::max_value().low_u128())); // how do we actually store U256?
		assert_eq!(basic.0.nonce, U256::from(0));
	})
}

#[test]
fn test_deploy_contract() {
    let (pairs, mut ext) = new_test_ext(1);
	ext.execute_with(|| {
		let xx = STORAGE_BYTECODE;
		println!("{:?}", xx)
	})
}
