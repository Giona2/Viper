use crate::error_handler::commands_error::CommandsErrorHandler;

use std::process::Command;
use colored::Colorize;

use super::{Commands, commands_error::CommandsError};


pub trait InProject{
	fn run(&self);
	fn list(&self);
	fn install(&self, args: Vec<String>);
	fn remove(&self, args: Vec<String>);

} impl InProject for Commands {
	fn run(&self) {


		let python_interpreter_dir = "venv/bin/python3";

		Command::new(python_interpreter_dir)
			.arg("./main.py")
            .status().expect("Failed to create main.py");
	}

	fn list(&self) {
        CommandsError::in_project_directory()
            .handle();

		let pip_dir = "./venv/bin/pip3";

		let package_list_raw = Command::new(pip_dir)
			.arg("freeze")
			.output().expect("Failed to get package list");

		print_freeze_output(String::from_utf8(package_list_raw.stdout).unwrap());
	}

	fn install(&self, args: Vec<String>) {
        CommandsError::in_project_directory()
            .handle();

		let pip_dir = "./venv/bin/pip3";
		let package_name = &args[0];

		println!("{} {} {}", "installing".yellow(), package_name.underline(), "package".yellow());
		Command::new(pip_dir)
			.args(["install", package_name])
            .status().expect(&format!("Failed to run pip install {package_name}"));
		println!("  {} {}", package_name.underline(), "installed".green())
	}

	fn remove(&self, args: Vec<String>) {
        CommandsError::in_project_directory()
            .handle();

		let pip_dir = "./venv/bin/pip3";
		let package_name = &args[0];

		println!("{} {} {}", "remove".yellow(), package_name.underline(), "package".yellow());
		Command::new(pip_dir)
			.args(["uninstall", package_name])
            .status().expect(&format!("Failed to install {package_name}"));
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
