use std::process::Command;
use std::path::Path;
use colored::Colorize;
use utils::type_traits::*;

use super::{Commands, Error};


pub trait InProject{
	fn run(&self);
	fn list(&self);
	fn install(&self, args: Vec<String>);
	fn remove(&self, args: Vec<String>);

} impl InProject for Commands {
	fn run(&self) {
		self.error_handler.handle(
			check_if_in_project(),
			"you are not currently in a project"
		);

		let python_interpreter_dir = "venv/bin/python3";

		let _ = self.error_handler.handle(
			Command::new(python_interpreter_dir)
				.arg("./main.py").status(),
			"failed to create main.py"
		);

	}

	fn list(&self) {
		self.error_handler.handle(
			check_if_in_project(),
			"you are not currently in a prject",
		);

		let pip_dir = "./venv/bin/pip3";

		let package_list_raw = self.error_handler.handle(
			Command::new(pip_dir)
				.arg("freeze")
				.output(),
			"failed to get package list"
		);

		print_freeze_output(String::from_utf8(package_list_raw.stdout).unwrap());
	}

	fn install(&self, args: Vec<String>) {
		self.error_handler.handle(
			check_if_in_project(),
			"your are not currently in a project"
		);

		let pip_dir = "./venv/bin/pip3";
		let package_name = &args[0];

		println!("{} {} {}", "installing".yellow(), package_name.underline(), "package".yellow());
		self.error_handler.handle(
			Command::new(pip_dir)
				.args(["install", package_name]).status(),
			&String::add_str(&["failed to install", package_name])
		);
		println!("  {} {}", package_name.underline(), "installed".green())
	}

	fn remove(&self, args: Vec<String>) {
		check_if_in_project()
			.red_expect("you are not currently in a project directory");

		let pip_dir = "./venv/bin/pip3";
		let package_name = &args[0];

		println!("{} {} {}", "remove".yellow(), package_name.underline(), "package".yellow());
		self.error_handler.handle(
			Command::new(pip_dir)
				.args(["uninstall", package_name]).status(),
			&String::add_str(&["failed to remove", package_name])
		);
		println!("  {} {}", package_name.underline(), "removed".green())
	}
}

fn print_freeze_output(freeze_output: String) {
	if freeze_output == String::new() {
		println!("{}", "There are no packages".green());
		return;
	}
	for package in freeze_output.trim().split("\n") {
		let package_name = package.split("==").collect::<Vec<&str>>()[0];
		let package_version = package.split("==").collect::<Vec<&str>>()[1];

		println!("{} ({})", package_name.trim().underline(), package_version.trim())
	}
}

fn check_if_in_project() -> Result<(), Error> {
	if !Path::new("./main.py").exists() || !Path::new("./venv/").exists() {
		Err(Error::NotInProjectDirectory)
	} else {
		Ok(())
	}
}

