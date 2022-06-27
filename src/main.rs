use airscript::do_file;
use clap::{command, arg};

fn main() {
  let matches = command!()
    .propagate_version(true)
    .arg(
      arg!(path: [PATH] "Path to AirScript file to execute")
      .required(true)
    )
    .get_matches();

  let filepath = matches.value_of("path").expect("path not provided");
  do_file(filepath).unwrap();
}
