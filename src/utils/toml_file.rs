use crate::error_handler::io_lib::IOLibHandler;

use std::path::Path;
use std::fs;
use toml;


pub struct TomlFile {
    path: String,
    content: toml::Table,

} impl TomlFile {
    pub fn new(path: &str) -> TomlFile {
        let mut content: toml::Table = toml::Table::new();

        if Path::new(path).exists() {
            let file_content: String = fs::read_to_string(path)   
                .handle(path);
            content = toml::from_str(&file_content)
                .expect(&format!("Failed to parse {path}"));
        } else {
            fs::write(path, toml::to_string::<toml::Table>(&content).unwrap())
                .handle(path);
        }

        return TomlFile { path: path.to_string(), content }
    }

    pub fn update_file(&self) {
        let file_content: String = toml::to_string::<toml::Table>(&self.content)
            .expect(&format!("Failed to parse {}", self.path));
        fs::write(&self.path, file_content)
            .handle(&self.path);
    }
}
