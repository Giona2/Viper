use super::ConfigFile;

use toml;


pub trait BuildSystem {
    fn create_build_system(&mut self);

} impl BuildSystem for ConfigFile {
    fn create_build_system(&mut self) {
        let mut build_system_table: toml::Table = toml::Table::new();

        let build_system_table_requires: toml::Value = toml::Value::Array(vec![
            toml::Value::String("pip".to_string()),
            toml::Value::String("venv".to_string()),
            toml::Value::String("viper".to_string()),
        ]);
        let build_system_table_build_backend: toml::Value = toml::Value::String("viper".to_string());

        if let Some(build_system_table) = self.file.content.get_mut("build-system") {
        }

        self.file.update_file();
    }
}
