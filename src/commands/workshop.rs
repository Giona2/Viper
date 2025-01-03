use crate::error_handler::{io_lib::IOLibHandler, commands_error::CommandsErrorHandler};

use std::fs;
use std::process::Command;
use crate::utils::type_traits::*;
use colored::Colorize;

use super::{Commands, commands_error::CommandsError, content};


pub trait InWorkshop {
	fn new(&self, args: Vec<String>);

} impl InWorkshop for Commands {
	fn new(&self, args: Vec<String>) {
		// Check if name is right
		let project_name: &str = &args[0];
        CommandsError::proper_characters_are_used(project_name)
            .handle();

		// Create project folder
		println!("{}", "creating project folder...".yellow());
		let project_dir = String::add_str(&["./", project_name, "/"]);
		fs::create_dir(&project_dir)
            .handle(&project_dir);
		println!("{}\n", "  project folder created".green());

		// Create venv folder
		println!("{}", "creating virtual environment...".yellow());
		let venv_dir = String::add_str(&[&project_dir, "venv/"]);
		Command::new("python3")
			.args(["-m", "venv", &venv_dir])
            .status().expect("failed to create virtual environment");
		println!("{}\n", "  virtual environment created".green());

		// Create main file
		println!("{}", "creating main.py".yellow());
		let main_python_dir = &String::add_str(&[&project_dir, "main.py"]);
		if args.len() == 1 {
			fs::write(main_python_dir, content::default_content())
                .expect("Failed to create main.py");
		} else { match args[1].as_str() {
			"-e" | "--entry-point" => { fs::write(main_python_dir, content::default_content())
                .expect("failed to create main.py");}

			"-c" | "--class"       => { fs::write(main_python_dir, content::class_content())
                .expect("failed to create main.py");}

			"-s" | "--simple"      => { fs::write(main_python_dir, content::simple_content())
                .expect("failed to create main.py");}

			_ => {}
		}}
		println!("{}\n", "  main.py created".green());
	}
}

