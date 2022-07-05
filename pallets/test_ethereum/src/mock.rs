use std::str::FromStr;

use frame_support::{
	parameter_types,
	sp_runtime::{
		app_crypto::sp_core::{H160, U256},
		testing::Header,
		testing::H256,
		traits::{BlakeTwo256, IdentityLookup},
		AccountId32,
	},
	traits::{ConstU16, ConstU64, GenesisBuild},
};
use pallet_ethereum::IntermediateStateRoot;
use pallet_evm::{AddressMapping, EnsureAddressNever, EnsureAddressRoot};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		EVM: pallet_evm::{Pallet, Call, Storage, Config, Event<T>},
		Ethereum: pallet_ethereum::{Pallet, Call, Storage, Event, Origin}
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}
impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub BlockGasLimit: U256 = U256::max_value();
}

pub struct HashedAddressMapping;

impl AddressMapping<AccountId32> for HashedAddressMapping {
	fn into_account_id(address: H160) -> AccountId32 {
		let mut data = [0u8; 32];
		data[0..20].copy_from_slice(&address[..]);
		AccountId32::from(Into::<[u8; 32]>::into(data))
	}
}

impl pallet_evm::Config for Test {
	type FeeCalculator = ();
	type GasWeightMapping = ();
	type CallOrigin = EnsureAddressRoot<Self::AccountId>;
	type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
	type AddressMapping = HashedAddressMapping;
	type Currency = Balances;
	type Event = Event;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type PrecompilesType = ();
	type PrecompilesValue = ();
	type ChainId = ();
	type OnChargeTransaction = ();
	type BlockGasLimit = BlockGasLimit;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type FindAuthor = ();
	type WeightInfo = ();
}

impl pallet_ethereum::Config for Test {
	type Event = Event;
	type StateRoot = IntermediateStateRoot<Self>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut accounts = std::collections::BTreeMap::new();
	accounts.insert(
		H160::from_str("1000000000000000000000000000000000000001").unwrap(),
		fp_evm::GenesisAccount {
			nonce: U256::from(0),
			balance: U256::max_value(),
			storage: std::collections::BTreeMap::<H256, H256>::new(),
			code: vec![],
		},
	);

	<pallet_evm::GenesisConfig as GenesisBuild<Test>>::assimilate_storage(
		&pallet_evm::GenesisConfig { accounts },
		&mut t,
	)
	.expect("Cannot assimilate storage");

	sp_io::TestExternalities::new(t)
}
