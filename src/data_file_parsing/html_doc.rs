use scraper;


pub struct HtmlElement {

}

pub enum HtmlElementType {
    MAIN,
    BODY,

}

pub struct HtmlDoc {
    pub contents: scraper::Html,

} impl HtmlDoc {
    pub fn new(contents: &str) -> HtmlDoc{ HtmlDoc {
        contents: scraper::Html::parse_document(contents)
    }}

    /// get_element(&["body", "main", 
    pub fn get_element(sequential_keys: &[&str]) -> String {
        fn get_element_recursive(html_doc: , sequential_keys: &[&str]) -> String {
        }

        String::new()
    }
}
