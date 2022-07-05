use std::str::FromStr;

use frame_support::sp_runtime::app_crypto::sp_core::H160;

use crate::mock::{new_test_ext, EVM};

#[test]
fn test_account_init() {
	new_test_ext().execute_with(|| {
		let evm_addr = H160::from_str("1000000000000000000000000000000000000001").unwrap();
		let is_account_empty = EVM::is_account_empty(&evm_addr);
        assert_eq!(is_account_empty, false)
	})
}
