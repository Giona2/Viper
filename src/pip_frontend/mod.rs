use crate::{data::PIP_DIR, main};

use std::process;
use scraper::{Html, Selector};


pub mod data;
use data::MatchedPackage;

const SEARCH_SITE: &str = "https://pydigger.com/search?q=";

use std::fs;
pub struct PipFrontend {
    pip_dir: String,
    search_site: String,
}
impl PipFrontend {
    pub fn new() -> PipFrontend { PipFrontend {
        pip_dir: PIP_DIR.to_string(),
        search_site: SEARCH_SITE.to_string(),
    }}

    /// Installs a package to the local venv using pip
    pub fn install(&self, package_name: &str) {
        process::Command::new(&self.pip_dir).arg("install")
            .arg(package_name)
            .status().unwrap();
    }

    /// Searches pypi.org to get all matches of the given package_name
    /// EXPERIMENTAL
    pub fn search(&self, package_name: &str) -> Vec<MatchedPackage> {
        //let client = reqwest::blocking::Client::new();
        //let search_response_content = //cont...
        //client.get(&format!("https://pydigger.com/search?q={package_name}"))
        //    .send().unwrap()
        //    .text().unwrap();
        let content = fs::read_to_string("./output.log")
            .unwrap();

        let tr_selector = Selector::parse("table.table tbody tr td").unwrap();
        let doc = Html::parse_document(&content);
        let table_rows: Vec<_> = doc.select(&tr_selector).collect();

        let mut result = Vec::new();
        for (i, _) in table_rows.iter().enumerate() {
            if (i+1) % 4 == 0 {
                let a_selector = Selector::parse("a").unwrap();
                let current_package_name_doc = Html::parse_fragment(&table_rows[i-3].inner_html());
                let current_package_name = current_package_name_doc.select(&a_selector).next();
                if let None = current_package_name { break; }

                let current_package = MatchedPackage::new(
                    &current_package_name.unwrap().inner_html(),
                    &table_rows[i-2].inner_html(),
                    &table_rows[i-1].inner_html()
                );
                result.push(current_package);
            }
        }
        
        return result
    }
}
