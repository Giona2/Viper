#[derive(Debug)]
pub struct MatchedPackage {
    pub name: String,
    pub version: String,
    pub description: String,

} impl MatchedPackage {
    pub fn new(name: &str, version: &str, description: &str) -> MatchedPackage { MatchedPackage {
        name: name.to_string(),
        version: version.to_string(),
        description: description.to_string(),
    }}
}
