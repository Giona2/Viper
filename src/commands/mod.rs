use colored::Colorize;
use utils::advanced_error_handling::*;


pub mod workshop;
pub mod project;
pub mod content;

pub struct Commands {
	pub error_handler: ErrorHandler,
} impl Commands {
	pub fn _new() -> Commands { Commands {
		error_handler: ErrorHandler::new(),
	}}

	pub fn help(&self, ) {
		println!("<{}> = arguments\n", "yellow".yellow());

		println!("viper new <{}>", "project name".yellow());
		println!("  creates a new python project with a virtual environment and a main.py file\n");
		println!("viper run");
		println!("  if in a folder created by viper new, it will run the main.py file sourcing the local virtual environment\n");
		println!("viper list");
		println!("  if in a folder created by viper new, it will return the list of installed packages in the local virtual environment\n  based on the pip3 freeze command\n");
		println!("viper install <{}>", "package name".yellow());
		println!("  if in a folder created by viper new, it will install the specified package to the local virtual environment\n  based on the pip3 install command\n");
		println!("viper remove <{}>", "package name".yellow());
		println!("  if in a folder created by viper new, it will remove the specified package from the local virtual environment\n  based on the pip3 uninstall command\n");

	}
}

#[derive(Debug, thiserror::Error)]
enum Error {
	#[error("Improper characters were used")]
	ImproperCharactersUsed,
	#[error("The user needs to run this command in a python project")]
	NotInProjectDirectory,
}
