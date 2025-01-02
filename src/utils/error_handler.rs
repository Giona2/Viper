use std::{fmt::Debug, process, result::Result};
use colored::Colorize;

pub struct ErrorHandler {
	display_rust_errors: bool
} impl ErrorHandler {
	pub fn new() -> ErrorHandler {
		ErrorHandler { display_rust_errors: false }
	}
	pub fn display_rust_errors(&mut self) {
		self.display_rust_errors = true
	}
	pub fn handle<T, E: Debug>(&self, handle: Result<T, E>, msg: &str) -> T {
		match handle {
			Ok(_) => handle.unwrap(),
			Err(e) => {
				println!("{}", msg.red());
				if self.display_rust_errors == true{ panic!("{e:?}") }
				else { process::exit(1) }
			}
		}
	}
}
