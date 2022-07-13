use clap::Parser;
use cli_opt::EthApi;
use sc_cli::RunCmd as RunCmdSubstrate;

#[derive(Debug, clap::Parser)]
// #[clap(
// 	propagate_version = true,
// 	args_conflicts_with_subcommands = true,
// 	subcommand_negates_reqs = true
// )]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run_eth: RunCmd,

	#[clap(flatten)]
	pub run: RunCmdSubstrate,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[clap(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Sub-commands concerned with benchmarking.
	#[clap(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,
}

#[derive(Debug, Parser)]
pub struct RunCmd {
	/// Public authoring identity to be inserted in the author inherent
	/// This is not currently used, but we may want a way to use it in the dev service.
	// #[clap(long)]
	// pub author_id: Option<NimbusId>,

	/// Enable EVM tracing module on a non-authority node.
	#[clap(
		long,
		conflicts_with = "collator",
		conflicts_with = "validator",
		use_value_delimiter = true,
		require_value_delimiter = true,
		multiple_values = true
	)]
	pub ethapi: Vec<EthApi>,

	/// Number of concurrent tracing tasks. Meant to be shared by both "debug" and "trace" modules.
	#[clap(long, default_value = "10")]
	pub ethapi_max_permits: u32,

	/// Maximum number of trace entries a single request of `trace_filter` is allowed to return.
	/// A request asking for more or an unbounded one going over this limit will both return an
	/// error.
	#[clap(long, default_value = "500")]
	pub ethapi_trace_max_count: u32,

	/// Duration (in seconds) after which the cache of `trace_filter` for a given block will be
	/// discarded.
	#[clap(long, default_value = "300")]
	pub ethapi_trace_cache_duration: u64,

	/// Size in bytes of the LRU cache for block data.
	#[clap(long, default_value = "300000000")]
	pub eth_log_block_cache: usize,

	/// Size in bytes of the LRU cache for transactions statuses data.
	#[clap(long, default_value = "300000000")]
	pub eth_statuses_cache: usize,

	/// Size in bytes of data a raw tracing request is allowed to use.
	/// Bound the size of memory, stack and storage data.
	#[clap(long, default_value = "20000000")]
	pub tracing_raw_max_memory_usage: usize,

	/// Maximum number of logs in a query.
	#[clap(long, default_value = "10000")]
	pub max_past_logs: u32,

	/// Force using Moonbase native runtime.
	#[clap(long = "force-moonbase")]
	pub force_moonbase: bool,

	/// Force using Moonriver native runtime.
	#[clap(long = "force-moonriver")]
	pub force_moonriver: bool,

	/// Id of the parachain this collator collates for.
	#[clap(long)]
	pub parachain_id: Option<u32>,

	/// Maximum fee history cache size.
	#[clap(long, default_value = "2048")]
	pub fee_history_limit: u64,
}

// impl std::ops::Deref for RunCmd {
// 	type Target = cumulus_client_cli::RunCmd;

// 	fn deref(&self) -> &Self::Target {
// 		&self.base
// 	}
// }
