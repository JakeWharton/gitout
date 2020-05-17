use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Args {
	/// Configuration file
	#[structopt(parse(from_os_str))]
	pub config: PathBuf,

	/// Backup directory
	#[structopt(parse(from_os_str))]
	pub destination: PathBuf,

	/// Enable verbose logging
	#[structopt(short, long)]
	pub verbose: bool,

	/// Print actions instead of performing them
	#[structopt(long)]
	pub dry_run: bool,
}

pub fn parse_args() -> Args {
	Args::from_args()
}
