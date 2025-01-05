use crate::io::toml::type_conversion::ValueToArray;
use crate::{error_handler::commands_error::CommandsErrorHandler, io::toml_file::TomlFile};
use crate::data;
use crate::io::toml::TomlExtra;

use std::process::Command;
use colored::Colorize;

use super::{Commands, commands_error::CommandsError};


pub trait InProject{
	fn run(&self);
	fn list(&self);
    fn reload(&self);
	//fn install(&self, args: Vec<String>);
	//fn remove(&self, args: Vec<String>);

} impl InProject for Commands {
	fn run(&self) {
        CommandsError::in_project_directory()
            .handle();

		Command::new(data::INTERPRETER_DIR)
			.arg("./main.py")
            .status().expect("Failed to run main.py");
	}

	fn list(&self) {
        CommandsError::in_project_directory()
            .handle();

		let package_list_raw = Command::new(data::PIP_DIR)
			.arg("freeze")
			.output().expect("Failed to get package list");

		print_freeze_output(String::from_utf8(package_list_raw.stdout).unwrap());
	}

    fn reload(&self) {
        CommandsError::in_project_directory()
            .handle();

        let mut config_file = TomlFile::new(data::CONFIG_FILE_NAME);

        let mut packages_to_intall = config_file.content.get_value(vec!["dependencies", "required"]);

        for package in packages_to_intall.get_array().unwrap() {
            let package_name: String = package.to_string()
                .replace("\"", "");

            Command::new(data::PIP_DIR).arg("install")
                .arg(package_name)
                .status().unwrap();
        }
    }

	/*fn install(&self, args: Vec<String>) {
        CommandsError::in_project_directory()
            .handle();

		let pip_dir = "./venv/bin/pip3";
		let package_name = &args[0];

		println!("{} {} {}", "installing".yellow(), package_name.underline(), "package".yellow());
		Command::new(pip_dir)
			.args(["install", package_name])
            .status().expect(&format!("Failed to run pip install {package_name}"));
		println!("  {} {}", package_name.underline(), "installed".green())
	}*/

	/*fn remove(&self, args: Vec<String>) {
        CommandsError::in_project_directory()
            .handle();

		let pip_dir = "./venv/bin/pip3";
		let package_name = &args[0];

		println!("{} {} {}", "remove".yellow(), package_name.underline(), "package".yellow());
		Command::new(pip_dir)
			.args(["uninstall", package_name])
            .status().expect(&format!("Failed to install {package_name}"));
		println!("  {} {}", package_name.underline(), "removed".green())
	}*/
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
