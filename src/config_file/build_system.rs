use super::ConfigFile;
use crate::io::toml::TomlExtra;

use toml;


pub trait BuildSystem {
    fn create_build_system(&mut self);

} impl BuildSystem for ConfigFile {
    fn create_build_system(&mut self) {
        self.file.content.insert_value(vec!["build-system", "requires"], toml::Value::Array(vec![
            toml::Value::String("pip".to_string()),
            toml::Value::String("venv".to_string()),
            toml::Value::String("viper".to_string()),
        ]));

        self.file.content.insert_value(vec!["build-system", "buld-backend"],
            toml::Value::String("viper".to_string())
        );

        self.file.update_file();
    }
}
