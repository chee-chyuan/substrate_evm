use std::str::FromStr;

use frame_support::sp_runtime::app_crypto::sp_core::{H160, U256};

use super::contracts::simple_storage::STORAGE_BYTECODE;
use crate::{
	mock::{address_build, new_test_ext, Ethereum, EVM},
	transactions::UnsignedTransaction,
};

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
fn test_transfer() {
	let (pairs, mut ext) = new_test_ext(1);
	let alice = &pairs[0];
	let bob = address_build(2);

	ext.execute_with(|| {
		let account_basic_alice_before = EVM::account_basic(&alice.address);
        let account_basic_bob_before = EVM::account_basic(&bob.address);

        let transfer_amount = U256::from(100);

		let tx_signed = UnsignedTransaction {
			nonce: account_basic_alice_before.0.nonce,
			max_priority_fee_per_gas: U256::from(1),
			max_fee_per_gas: U256::from(1),
			gas_limit: U256::from(0x100000),
			action: ethereum::TransactionAction::Call(H160::from(bob.address)),
			value: transfer_amount,
			input: vec![],
		}
		.sign(&alice.private_key, Some(123 as u64));

        // println!("{:?}", tx_signed);

        let res = Ethereum::execute(alice.address, &tx_signed, None);
        res.unwrap();


        let account_basic_bob_after = EVM::account_basic(&bob.address);
        assert_eq!(account_basic_bob_after.0.balance, transfer_amount);

		let account_basic_alice_after = EVM::account_basic(&alice.address);
        // assert_eq!(account_basic_alice_after.0.balance, account_basic_alice_before.0.balance - transfer_amount);
        println!("alice before: {:?}", account_basic_alice_before);
        println!("alice after: {:?}", account_basic_alice_after);
	})
}

// #[test]
// fn test_deploy_contract() {
//     let (pairs, mut ext) = new_test_ext(1);
//     let alice = &pairs[0];
// 	ext.execute_with(|| {
// 		let xx = STORAGE_BYTECODE;
// 		println!("{:?}", xx)
// 	})
// }
