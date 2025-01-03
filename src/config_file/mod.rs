use crate::data;

use std::path::Path;


pub mod build_system;
use build_system::*;
pub mod dependencies;
use dependencies::*;
pub mod metadata;
use metadata::*;
mod toml_file;
use toml_file::TomlFile;

pub struct ConfigFile {
    file: TomlFile,

} impl ConfigFile {
    pub fn new() -> ConfigFile {
        let mut result: ConfigFile = ConfigFile { file: TomlFile::new(data::CONFIG_FILE_DIR) };

        if !Path::new(data::CONFIG_FILE_DIR).exists() {
            result.file.content.insert("build-system".to_string(), toml::Value::Table(toml::Table::new()));
            result.file.content.insert("project".to_string(), toml::Value::Table(toml::Table::new()));
            result.file.content.insert("dependencies".to_string(), toml::Value::Table(toml::Table::new()));

            result.create_metadata();
            result.create_build_system();
            result.create_dependencies();
        }

        return result;
    }
}
