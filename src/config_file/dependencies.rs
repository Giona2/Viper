use super::ConfigFile;


pub trait Dependencies {
    fn create_dependencies(&mut self);

} impl Dependencies for ConfigFile {
    fn create_dependencies(&mut self) {
    }
}
