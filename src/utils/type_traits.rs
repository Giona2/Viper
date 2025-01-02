use std::fmt::Debug;

use colored::Colorize;

pub trait StringExtra {
	fn split_string_by_increment(&self, increment: usize) -> Vec<String>;
	fn add_str(strs: &[&str]) -> String;
}
impl StringExtra for String {
	fn split_string_by_increment(&self, increment: usize) -> Vec<String> {
		let mut self_to_chars: Vec<char> = self.chars().collect();
		let mut result: Vec<String> = Vec::new();

		while self_to_chars.len() >= increment {
			let char_vec: Vec<char> = self_to_chars[0..increment].to_vec();
			result.push(char_vec.to_string());
			self_to_chars = self_to_chars[increment..self_to_chars.len()].to_vec();
		}
		let char_vec: Vec<char> = self_to_chars[0..self_to_chars.len()].to_vec();
		result.push(char_vec.to_string());

		result
	}
	fn add_str(strs: &[&str]) -> String {
		let mut result = String::new();

		for str in strs {
			result += str
		}

		result
	}
}

pub trait CharVecExtra {
	fn to_string(&self) -> String;

} impl CharVecExtra for Vec<char> {
	fn to_string(&self) -> String {
		let mut result = String::new();
		for character in self {
			result += character.to_string().as_str()
		}
		result
	}
}

pub trait ResultExtra<T> {
	fn red_expect(self, msg: &str) -> T;
	fn red_expect_explicit(self, msg: &str) -> T;

} impl<T, E: Debug> ResultExtra<T> for Result<T, E> {
	fn red_expect(self, msg: &str) -> T { match self {
		Ok(_) => self.unwrap(),
		Err(e) => {
			println!("{}", msg.red());
			panic!("{e:?}")
		}
	}}
	fn red_expect_explicit(self, msg: &str) -> T { match self {
		Ok(_) => self.unwrap(),
		Err(_) => {
			println!("{}", msg.red());
			std::process::exit(1)
		}
	}}
}
