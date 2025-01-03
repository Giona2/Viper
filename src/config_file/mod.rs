use crate::utils::toml_file::TomlFile;


pub mod build_system;
use build_system::*;
pub mod dependencies;
use dependencies::*;
pub mod metadata;
use metadata::*;

pub struct ConfigFile {
    file: TomlFile,

} impl ConfigFile {
    pub fn new(path: &str) -> ConfigFile {
        let mut result: ConfigFile = ConfigFile { file: TomlFile::new(path) };
        
        result.file.content.insert("build-system".to_string(), toml::Value::Table(toml::Table::new()));
        result.file.content.insert("project".to_string(), toml::Value::Table(toml::Table::new()));

        result.create_metadata();
        result.create_build_system();
        result.create_dependencies();

        return result;
    }
}
