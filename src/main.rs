use std::fs;

mod args;
mod config;

fn main() {
  let args = args::parse_args();
  if args.verbose {
    dbg!(&args);
  }
  let args::Args {
    config,
    destination,
    verbose,
  } = args;

  let destination_metadata = fs::metadata(&destination).unwrap();
  if !destination_metadata.is_dir() {
    panic!("Destination must exist and must be a directory")
  }

  let config = fs::read_to_string(config).unwrap();
  let config = config::parse_config(&config).unwrap();
  if verbose {
    dbg!(&config);
  }
}
