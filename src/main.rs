//! A minimal Rust binary used as the starting point for new projects.

fn main() {
	println!("{}", greeting("world"));
}

/// Build a friendly greeting for `name`.
fn greeting(name: &str) -> String {
	format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
	use super::greeting;

	#[test]
	fn greets_by_name() {
		assert_eq!(greeting("world"), "Hello, world!");
	}
}
