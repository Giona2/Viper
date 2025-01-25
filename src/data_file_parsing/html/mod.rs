use scraper::{Html, selector::Selector, ElementRef};


pub mod data;
use data::HtmlElmt;

pub trait HtmlExtra {
    fn get_element(&self, sequential_keys: &[HtmlElmt]) -> String;

} impl HtmlExtra for Html {
    /// get_element(&[
    ///     HtmlElmt::new("body", None),
    ///     HtmlElmt::new("main", None),
    ///     HtmlElmt::new("div", vec![
    ///         HtmlAttr{name: "class", val: "meh"},
    ///     ]),
    ///     ...
    /// ])
    fn get_element(&self, sequential_keys: &[HtmlElmt]) -> String {
        fn get_element_recursive(html_doc: &Html, sequential_keys: &[HtmlElmt]) -> String {
            //println!();
            //println!("name: {}, index: {}", sequential_keys[0].name, sequential_keys[0].index);

            if sequential_keys.len() > 1 {
                let selector = Selector::parse(&sequential_keys[0].name).unwrap();
                let applied_elements: Vec<ElementRef<'_>> = html_doc.select(&selector).collect();
                //println!("applied elements: {:?}", applied_elements);
                //println!("index 2: {}", sequential_keys[0].index);
                let current_element_contents = applied_elements[sequential_keys[0].index].inner_html();
                let next_doc = Html::parse_fragment(&current_element_contents);

                get_element_recursive(&next_doc, &sequential_keys[1..])
            } else {
                let selector = Selector::parse(&sequential_keys[0].name).unwrap();
                let applied_elements: Vec<ElementRef<'_>> = html_doc.select(&selector).collect::<>();
                //println!("applied elements: {:?}", applied_elements);
                //println!("index 2: {}", sequential_keys[0].index);

                return applied_elements[sequential_keys[0].index].inner_html();
            }
        }
        
        get_element_recursive(self, sequential_keys)
    }
}
