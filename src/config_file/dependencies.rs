use crate::io::toml::TomlExtra;

use super::ConfigFile;


pub trait Dependencies {
    fn create_dependencies(&mut self);
    fn install_dependencies(&self);

} impl Dependencies for ConfigFile {
    fn create_dependencies(&mut self) {
        self.file.content.insert_value(vec!["dependencies", "required"],
            toml::Value::Array(Vec::new())
        );
    }

    fn install_dependencies(&self) {
    }
}
