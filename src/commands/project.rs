use crate::data_file_parsing::toml::type_conversion::*;
use crate::data_file_parsing::toml_file::TomlFile;
use crate::data;
use crate::data_file_parsing::toml::TomlExtra;
use crate::pip_frontend::PipFrontend;
use crate::error_handler::python::PythonHandler;

use std::process;

use super::Commands;


pub trait InProject{
	fn run(&self, args: Option<Vec<String>>);
    fn reload(&self);
    fn search(&self, package_name: &str);

} impl InProject for Commands {
	fn run(&self, args: Option<Vec<String>>) {
        // Error handling
        if args.clone() != None && args.clone().unwrap()[0] != "--" {
            println!("unknown argument after run\nmake sure to use -- before passing in arguments for main.py");
            process::exit(1);
        }
        if args.clone() != None && args.clone().unwrap().len() < 1 {
            println!("some args are missing\ntype viper help to see how to use this command");
            process::exit(1);
        }

        if let Some(unwrapped_args) = args {
		    process::Command::new(data::INTERPRETER_DIR)
			    .arg(&(data::SOURCE_FILES_DIR.to_owned() + "/main.py"))
                .args(unwrapped_args)
                .status().python_handle();
        } else {
		    process::Command::new(data::INTERPRETER_DIR)
			    .arg(&(data::SOURCE_FILES_DIR.to_owned() + "/main.py"))
                .status().python_handle();
        }
	}

    fn reload(&self) {
        let mut config_file = TomlFile::new(data::CONFIG_FILE_NAME);
        let mut viper_config_file = TomlFile::new(data::VIPER_CONFIG_FILE_DIR); 

        let installed_packages = viper_config_file.content.get_mut("installed_packages").unwrap()
            .as_array_mut().unwrap();

        // Install Packages
        let mut packages_to_install = config_file.content.get_value(vec!["dependencies", "required"]);

        for package in packages_to_install.get_array().unwrap() {
            let package_name: String = package.to_string()
                .replace("\"", "");

            self.pip.install(&package_name);

            installed_packages.push(toml::Value::String(package_name));
        }


        // Remove Packages
        let mut updated_installed_packages: Vec<toml::Value> = Vec::new();

        for installed_package in installed_packages.clone() {

            if !packages_to_install.get_array().unwrap().contains(&installed_package) {
                let installed_package_name: String = installed_package.to_string()
                    .replace("\"", "");
                
                self.pip.remove(&installed_package_name);
                
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

    fn search(&self, package_name: &str) {
        let pip_frontend = PipFrontend::new();
        
        for package in pip_frontend.search(package_name) {
            println!("{}: {}", package.name, package.version);
            println!(" 󱞩 {}", package.description);
            println!();
        }
    }
}
