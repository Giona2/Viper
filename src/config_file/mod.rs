pub mod build_system;
pub mod dependencies;
pub mod metadata;


pub struct ConfigFile {
    path: String

} impl ConfigFile {
    pub fn new() -> ConfigFile { return ConfigFile {
        path: "./pyproject.toml".to_string(),
    }}
}
