//! A minimal Rust binary used as the starting point for new projects.

use clap::Parser;

/// Command-line arguments.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
	/// Name to greet.
	#[arg(default_value = "world")]
	name: String,
}

fn main() {
	let args = Args::parse();
	println!("{}", greeting(&args.name));
}

/// Build a friendly greeting for `name`.
fn greeting(name: &str) -> String {
	format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
	use super::{Args, greeting};
	use clap::Parser;

	#[test]
	fn greets_by_name() {
		assert_eq!(greeting("world"), "Hello, world!");
	}

	#[test]
	fn parses_name_argument() {
		let args = Args::try_parse_from(["rust-template", "Alice"]).unwrap();
		assert_eq!(args.name, "Alice");
	}

	#[test]
	fn name_defaults_to_world() {
		let args = Args::try_parse_from(["rust-template"]).unwrap();
		assert_eq!(args.name, "world");
	}
}
