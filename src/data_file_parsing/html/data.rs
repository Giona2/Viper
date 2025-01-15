pub struct HtmlElmt {
    pub html_elmt_type: String,
} impl HtmlElmt {
    pub fn new(html_elmt_type: &str) -> HtmlElmt { HtmlElmt {
        html_elmt_type: html_elmt_type.to_string(),
    }}
}
