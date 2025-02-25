use std::{env::args, process};

mod commands;
use commands::{Commands, workshop::*, project::*};
mod error_handler;
mod data_file_parsing;
mod pip_frontend;
mod data;


fn main() {
	let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("unknown command given\ntype viper help to see how to use this command");
        process::exit(1)
    }

	let commands = Commands::_new();
	match args[1].as_str() {
		"new"		 => commands.new(args[2..args.len()].to_vec()),
        
		"run"		 => {
            if args.len() > 2 { commands.run(Some(args[2..args.len()].to_vec())) }
            else              { commands.run(None) }
        },

        "reload"     => commands.reload(),
        "search"     => commands.search(&args[2]),

		"help" | "h" => commands.help(),
		           _ => {println!("unknown command given\ntype viper help to see how to use this command"); process::exit(1)}
	}
}
