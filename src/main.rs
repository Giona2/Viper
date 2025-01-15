use std::env::args;

mod commands;
use commands::{Commands, workshop::*, project::*};
mod error_handler;
mod data_file_parsing;
mod pip_frontend;
mod data;


fn _main() {
	let args: Vec<String> = args().collect();

	let commands = Commands::_new();
	match args[1].as_str() {
		"new"		 => commands.new(args[2..args.len()].to_vec()),
        
		"run"		 => commands.run(),
		"list"		 => commands.list(),
        "reload"     => commands.reload(),

		"help" | "h" => commands.help(),
		_ => {}
	}
}


const WEBSITE: &str = 
r#"
<body>
    <ul>
        <li class="meh">hey</li>
        <li>hello</li>
    </ul>
</body>
"#;

use data_file_parsing::html::{data::*, HtmlExtra};
use scraper::Html;
fn main() {
    let html_doc = Html::parse_document(WEBSITE);
    let div = html_doc.get_element(&[
        HtmlElmt::new("body",   None),
        HtmlElmt::new("ul",     None),
        HtmlElmt::new("li.meh", None),
    ]);

    println!("{:?}", div)
}
