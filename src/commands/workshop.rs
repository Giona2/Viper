use crate::data;
use crate::error_handler::io_lib::IOLibHandler;
use crate::error_handler::python::PythonHandler;
use crate::data_file_parsing::toml_file::TomlFile;
use crate::data_file_parsing::toml::TomlExtra;

use std::fs;
use std::process::Command;
use colored::Colorize;

use super::{Commands, content};


pub trait InWorkshop {
	fn new(&self, args: Vec<String>);

} impl InWorkshop for Commands {
	fn new(&self, args: Vec<String>) {
		// Check if name is right
		let project_name: &str = &args[0];

		// Create project folder
		println!("{}", "creating project folder...".yellow());
		let project_dir = format!("./{project_name}");
		fs::create_dir(&project_dir)
            .io_lib_handle(&project_dir);
		println!("{}\n", "  project folder created".green());

        // Create ConfigFile
		println!("{}", "creating config file...".yellow());
        let config_file_dir: String = format!("{project_dir}/{}", data::CONFIG_FILE_NAME);
        let mut config_file: TomlFile = TomlFile::new(&config_file_dir);
        config_file.content = toml::from_str(content::CONFIG_FILE)
            .unwrap();
        config_file.content.insert_value(vec!["build-system", "requires"], toml::Value::Array(vec![
            toml::Value::String("pip".to_string()),
            toml::Value::String("venv".to_string()),
            toml::Value::String("viper".to_string()),
        ]));
        config_file.content.insert_value(vec!["project", "name"], toml::Value::String(
            project_name.to_string()
        ));
        config_file.content.insert_value(vec!["project", "version"], toml::Value::String(
            "1.0.0".to_string()
        ));
        config_file.content.insert_value(vec!["dependencies", "required"], toml::Value::Array(
            Vec::new(),
        ));
        config_file.update_file();
		println!("{}\n", "  config file created".green());

		// Create venv folder
		println!("{}", "creating virtual environment...".yellow());
		let venv_dir = format!("{project_dir}/venv");
		Command::new("python")
			.args(["-m", "venv", &venv_dir])
            .status().python_handle();
		println!("{}\n", "  virtual environment created".green());

        // Create viper config folder and installed packages file
        let viper_config_dir = format!("{project_dir}/venv/lib/viper");
        fs::create_dir(&viper_config_dir)
            .io_lib_handle(&viper_config_dir);
        let installed_packages_dir = format!("{viper_config_dir}/config.toml");
        let mut installed_packages_file: TomlFile = TomlFile::new(&installed_packages_dir);
        installed_packages_file.content.insert_value(vec!["installed_packages"], toml::Value::Array(Vec::new()));
        installed_packages_file.update_file();

        // Create src folder
		println!("{}", "creating src directory...".yellow());
        let src_dir = format!("{project_dir}/src");
        fs::create_dir(&src_dir)
            .io_lib_handle(&src_dir);
		println!("{}\n", "  src directory created".green());

		// Create main file
		println!("{}", "creating main.py".yellow());
		let main_python_dir: String = format!("{src_dir}/main.py");
		if args.len() == 1 {
			fs::write(main_python_dir, content::ENTRY_POINT)
                .expect("Failed to create main.py");
		} else { match args[1].as_str() {
			"-e" | "--entry-point" => { fs::write(main_python_dir, content::ENTRY_POINT)
                .expect("failed to create main.py");}

			"-c" | "--class"       => { fs::write(main_python_dir, content::CLASS)
                .expect("failed to create main.py");}

			"-s" | "--simple"      => { fs::write(main_python_dir, content::SIMPLE)
                .expect("failed to create main.py");}

			_ => {}
		}}
		println!("{}\n", "  main.py created".green());
	}
}

