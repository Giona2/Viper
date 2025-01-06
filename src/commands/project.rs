use crate::data_file_parsing::toml::type_conversion::*;
use crate::{error_handler::commands_error::CommandsErrorHandler, data_file_parsing::toml_file::TomlFile};
use crate::data;
use crate::data_file_parsing::toml::TomlExtra;

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
			.arg(&(data::SOURCE_FILES_DIR.to_owned() + "/main.py"))
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
        let mut viper_config_file = TomlFile::new(data::VIPER_CONFIG_FILE_DIR); 

        let installed_packages = viper_config_file.content.get_mut("installed_packages").unwrap()
            .as_array_mut().unwrap();

        // Install Packages
        let mut packages_to_install = config_file.content.get_value(vec!["dependencies", "required"]);

        for package in packages_to_install.get_array().unwrap() {
            let package_name: String = package.to_string()
                .replace("\"", "");

            Command::new(data::PIP_DIR).arg("install")
                .arg(&package_name)
                .status().unwrap();

            installed_packages.push(toml::Value::String(package_name));
        }


        // Remove Packages
        let mut updated_installed_packages: Vec<toml::Value> = Vec::new();

        for installed_package in installed_packages.clone() {

            if !packages_to_install.get_array().unwrap().contains(&installed_package) {
                let installed_package_name: String = installed_package.to_string()
                    .replace("\"", "");
                
                Command::new(data::PIP_DIR).arg("uninstall")
                    .arg(installed_package_name)
                    .arg("-y")
                    .status().unwrap();
                
            } else {
                let installed_package_name: String = installed_package.to_string()
                    .replace("\"", "");

                updated_installed_packages.push(toml::Value::String(installed_package_name));
            }
        }

        installed_packages.clear();
        installed_packages.extend(updated_installed_packages);


        viper_config_file.update_file();
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
