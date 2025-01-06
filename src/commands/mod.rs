use colored::Colorize;


pub mod workshop;
pub mod project;
pub mod content;
pub mod commands_error;

pub struct Commands {
} impl Commands {
	pub fn _new() -> Commands { Commands {
	}}

	pub fn help(&self, ) {
		println!("--<{}>   = tags",    "blue".blue());
		println!("  <{}> = arguments", "yellow".yellow());
	    println!();
        println!("viper new <{}> --<{}>", "project name".yellow(), "main.py layout".blue());
		println!("  creates a new python project with a virtual environment and a main.py file. You can pass in a few arguments to get different main.py layouts");
	    println!("    -e | --entry-point: Creates the main.py in the entry point layout");
	    println!("    -c | --class      : Creates the main.py in the object-oriented layout");
	    println!("    -s | --simple     : Creates the main.py in the most simplistic layout as possible");
	    println!();
		println!("viper run");
		println!("  if in a folder created by viper new, it will run the main.py file sourcing the local virtual environment");
	    println!();
        println!("viper reload");
        println!("  if in a folder created by viper new, it will reload the packages listed under the [dependencies] section of the pyproject.toml file and install/remove packages as needed
")
	}
}
