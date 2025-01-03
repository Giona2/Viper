use crate::error_handler::io_lib::IOLibHandler;

use std::path::Path;
use std::fs;
use toml;


pub mod type_conversion;
use type_conversion::toml_table::TomlConvertTomlTable;

pub struct TomlFile {
    pub path: String,
    pub content: toml::Table,

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

    ///[first-section]
    ///[first-section.child]
    ///val = "3"
    ///
    /// change_value(vec!["first-section", "child", "val"], toml::Value::String("5".to_string))
    pub fn change_value(mut self, sequential_keys: Vec<&str>, value: toml::Value) {
        fn change_value_recursive(toml_table: &mut toml::Table, sequential_keys: Vec<&str>, value: toml::Value) {
            let current_table: &mut toml::Table = toml_table.get_mut(sequential_keys[0]).unwrap()
                .get_toml_table().unwrap();

            if sequential_keys.len() > 1 {
                change_value_recursive(current_table, sequential_keys[1..].to_vec(), value);
            } else {
                current_table.insert(sequential_keys[0].to_string(), value);
            }
        }

        change_value_recursive(&mut self.content, sequential_keys, value);
    }
}

pub trait TomlExtra {
    fn change_value(&mut self, sequential_keys: Vec<&str>, value: toml::Value);

} impl TomlExtra for toml::Table {
    fn change_value(&mut self, sequential_keys: Vec<&str>, value: toml::Value) {
        fn change_value_recursive(toml_table: &mut toml::Table, sequential_keys: Vec<&str>, value: toml::Value) {
            println!("Sequential Keys: {sequential_keys:?}");

            if sequential_keys.len() > 1 {
                let next_table: &mut toml::Table = toml_table.get_mut(sequential_keys[0]).unwrap()
                    .get_toml_table().unwrap();
                change_value_recursive(next_table, sequential_keys[1..].to_vec(), value);
            } else {
                toml_table.insert(sequential_keys[0].to_string(), value);
            }
        }

        change_value_recursive(self, sequential_keys, value);
    }
}
