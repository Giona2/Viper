use crate::data;

use std::process;
use reqwest;
use scraper::{self, selectable::Selectable};


const SEARCH_SITE: &str = "https://www.pypi.org/";
const SEARCH_PROMPT_SUFFIX: &str = "search/?q=";

pub struct PipFrontend {
    pip_dir: String,
    search_site: String,
    search_prompt_suffix: String,
}
impl PipFrontend {
    pub fn new() -> PipFrontend { PipFrontend {
        pip_dir: data::PIP_DIR.to_string(),
        search_site: SEARCH_SITE.to_string(),
        search_prompt_suffix: SEARCH_PROMPT_SUFFIX.to_string(),
    }}

    /// Installs a package to the local venv using pip
    pub fn install(&self, package_name: &str) {
        process::Command::new(&self.pip_dir).arg("install")
            .arg(package_name)
            .status().unwrap();
    }

    /// Searches pypi.org to get all matches of the given package_name
    pub fn search(&self, package_name: &str) -> Vec<&str> {
        // Selectors
        let ul_selector   = scraper::Selector::parse("ul").unwrap();
        let body_selector = scraper::Selector::parse("body").unwrap();
        let main_selector = scraper::Selector::parse("main").unwrap();


        let client = reqwest::blocking::Client::new();
        println!("{}", self.search_site.clone() + &self.search_prompt_suffix + package_name);
        let body = client.get(&(self.search_site.clone() + &self.search_prompt_suffix + package_name))
            .send().unwrap()
            .text().unwrap();

        let document = scraper::Html::parse_document(&body);
        let body_element: scraper::ElementRef = document.select(&body_selector).next().unwrap();
        println!("ul_elements: {:?}", body_element);
        let body_children: String = body_element.text().collect();
        println!("ul_children: {:?}", body_children);

        Vec::new()
    }
}
