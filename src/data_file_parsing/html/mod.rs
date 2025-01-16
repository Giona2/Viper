use scraper::{Html, selector::Selector};


pub trait HtmlExtra {
    fn get_element(&self, sequential_keys: &[&str]) -> String;

} impl HtmlExtra for Html {
    /// get_element(&[
    ///     HtmlElmt::new("body", None),
    ///     HtmlElmt::new("main", None),
    ///     HtmlElmt::new("div", vec![
    ///         HtmlAttr{name: "class", val: "meh"},
    ///     ]),
    ///     ...
    /// ])
    fn get_element(&self, sequential_keys: &[&str]) -> String {
        fn get_element_recursive(html_doc: &Html, sequential_keys: &[&str]) -> String {
            println!("Current element: {}", sequential_keys[0]);
            println!("Current Doc: {}\n", html_doc.html());

            if sequential_keys.len() > 1 {
                let selector = Selector::parse(&sequential_keys[0]).unwrap();
                let current_element = &html_doc.select(&selector).next().unwrap();
                let current_element_contents = current_element.inner_html();
                let next_doc = Html::parse_fragment(&current_element_contents);

                get_element_recursive(&next_doc, &sequential_keys[1..])
            } else {
                let selector = Selector::parse(&sequential_keys[0]).unwrap();
                let current_element = &html_doc.select(&selector).next().unwrap();

                return current_element.inner_html();
            }
        }
        
        get_element_recursive(self, sequential_keys)
    }
}
