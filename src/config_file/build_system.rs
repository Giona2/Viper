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

        build_system_table.insert("requires".to_string(), build_system_table_requires);
        build_system_table.insert("build_backend".to_string(), build_system_table_build_backend);

        self.file.content.insert("build-system".to_string(), toml::Value::Table(build_system_table));
        self.file.update_file();
    }
}
