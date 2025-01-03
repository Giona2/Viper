pub mod type_conversion;
use type_conversion::ValueToTable;

pub trait TomlExtra {
    fn insert_value(&mut self, sequential_keys: Vec<&str>, value: toml::Value);

} impl TomlExtra for toml::Table {
    /// Moves through each level of the table for each key in sequential_keys until the last value
    /// is found and replaced/inserted
    ///
    /// The python dict equivalent would be as follows:
    /// table.insert_value(
    ///     vec!["first_entry", "child_entry", "grandchild"],
    ///     toml::Value::String("uno".to_string)
    /// )
    /// table["first_entry"]["child_entry"]["grandchild"] = "uno"
    fn insert_value(&mut self, sequential_keys: Vec<&str>, value: toml::Value) {
        fn insert_value_recursive(toml_table: &mut toml::Table, sequential_keys: Vec<&str>, value: toml::Value) {
            println!("Sequential Keys: {sequential_keys:?}");

            if sequential_keys.len() > 1 {
                let next_table: &mut toml::Table = toml_table.get_mut(sequential_keys[0]).unwrap()
                    .get_toml_table().unwrap();
                insert_value_recursive(next_table, sequential_keys[1..].to_vec(), value);
            } else {
                toml_table.insert(sequential_keys[0].to_string(), value);
            }
        }

        insert_value_recursive(self, sequential_keys, value);
    }
}
