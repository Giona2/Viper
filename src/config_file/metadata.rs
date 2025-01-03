use super::ConfigFile;


pub trait Metadata {
    fn create_metadata(&mut self);
    
} impl Metadata for ConfigFile {
    fn create_metadata(&mut self) {
    }
}
