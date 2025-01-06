use toml;


pub trait ValueToTable {
    fn get_toml_table(&mut self) -> Option<&mut toml::Table>;

} impl ValueToTable for toml::Value {
    fn get_toml_table(&mut self) -> Option<&mut toml::Table> { if let Some(table) = self.as_table_mut(){
        return Some(table)
    } else { return None }}
}

pub trait ValueToArray {
    fn get_array(&mut self) -> Option<&mut Vec::<toml::Value>>;

} impl ValueToArray for toml::Value {
    fn get_array(&mut self) -> Option<&mut Vec::<toml::Value>> { if let Some(array) = self.as_array_mut(){
        return Some(array)
    } else { return None }}
}
