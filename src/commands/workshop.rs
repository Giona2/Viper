use std::fs;
use std::process::Command;
use crate::utils::type_traits::*;
use colored::Colorize;

use super::{Commands, Error, content};


pub trait InWorkshop {
	fn new(&self, args: Vec<String>);

} impl InWorkshop for Commands {
	fn new(&self, args: Vec<String>) {
		// Check if name is right
		let project_name: &str = &args[0];
		self.error_handler.handle(
			check_for_wrong_characters(project_name),
			"Improper characters were used"
		);

		// Create project folder
		println!("{}", "creating project folder...".yellow());
		let project_dir = String::add_str(&["./", project_name, "/"]);
		self.error_handler.handle(
			fs::create_dir(&project_dir),
			"failed to create project folder"
		);
		println!("{}\n", "  project folder created".green());

		// Create venv folder
		println!("{}", "creating virtual environment...".yellow());
		let venv_dir = String::add_str(&[&project_dir, "venv/"]);
		self.error_handler.handle(
			Command::new("python3")
				.args(["-m", "venv", &venv_dir]).status(),
			"failed to create virtual environment"
		);
		println!("{}\n", "  virtual environment created".green());

		// Create main file
		println!("{}", "creating main.py".yellow());
		let main_python_dir = &String::add_str(&[&project_dir, "main.py"]);
		if args.len() == 1 { self.error_handler.handle(
			fs::write(main_python_dir, content::default_content()),
			"failed to create main.py"

		);} else { match args[1].as_str() {
			"-e" | "--entry-point" => { self.error_handler.handle(
				fs::write(main_python_dir, content::default_content()),
				"failed to create main.py"
			);}
			"-c" | "--class"       => { self.error_handler.handle(
				fs::write(main_python_dir, content::class_content()),
				"failed to create main.py"
			);}
			"-s" | "--simple"      => { self.error_handler.handle(
				fs::write(main_python_dir, content::simple_content()),
				"failed to create main.py"
			);}
			_ => {}
		}}
		println!("{}\n", "  main.py created".green());

	}
}

fn check_for_wrong_characters(project_name: &str) -> Result<(), Error> {
	for character in project_name.chars() { match character {
		'#' => return Err(Error::ImproperCharactersUsed),
		'%' => return Err(Error::ImproperCharactersUsed),
		'&' => return Err(Error::ImproperCharactersUsed),
		'{' => return Err(Error::ImproperCharactersUsed),
		'}' => return Err(Error::ImproperCharactersUsed),
		'<' => return Err(Error::ImproperCharactersUsed),
		'>' => return Err(Error::ImproperCharactersUsed),
		'*' => return Err(Error::ImproperCharactersUsed),
		'?' => return Err(Error::ImproperCharactersUsed),
		'/' => return Err(Error::ImproperCharactersUsed),
		'$' => return Err(Error::ImproperCharactersUsed),
		'!' => return Err(Error::ImproperCharactersUsed),
		'"' => return Err(Error::ImproperCharactersUsed),
		':' => return Err(Error::ImproperCharactersUsed),
		'@' => return Err(Error::ImproperCharactersUsed),
		'+' => return Err(Error::ImproperCharactersUsed),
		'`' => return Err(Error::ImproperCharactersUsed),
		'|' => return Err(Error::ImproperCharactersUsed),
		'=' => return Err(Error::ImproperCharactersUsed),
		 _  => return Ok(())
	}};
	Ok(())
}
