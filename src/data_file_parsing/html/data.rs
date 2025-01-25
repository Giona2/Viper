pub struct HtmlElmt{
    pub name: String,
    pub index: usize,

} impl HtmlElmt {
    pub fn new(index: usize, name: &str, ) -> HtmlElmt { HtmlElmt{
        name: name.to_string(),
        index
    }}
}
