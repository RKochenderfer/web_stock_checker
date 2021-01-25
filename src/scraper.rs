use crate::site::Site;

use lazy_static::lazy_static;
use scraper::{Html, Selector};
use std::collections::HashMap;

lazy_static! {
    /// A map containing the sites that are available to be checked
    static ref SITE_MAP: HashMap<String, Site> = {
        let mut m = HashMap::new();
        m.insert("newegg".to_string(), Site::NEWEGG);

        m
    };
    /// A map containing the site information
    static ref DIV_MAP: HashMap<Site, SiteInfo> = {
        let mut m = HashMap::new();
        m.insert(Site::NEWEGG, SiteInfo {
            stock_status_tag: TargetTagType::Class,
            // out_of_stock_div: r#"<strong><i class="fas fa-exclamation-triangle"></i> OUT OF STOCK.</strong>"#,
            out_of_stock_div: "strong",
            stock_status_selector: "product-inventory",
            out_of_stock_text: "OUT OF STOCK",
            item_title_selector_type: TargetTagType::Class,
            item_title_selector: "product-title"
        });

        m
    };
}

#[derive(Debug, Copy, Clone)]
pub enum TargetTagType {
    Class,
    ID,
}

/// The web scraper
#[derive(Debug, Copy, Clone)]
pub struct Scraper<'a> {
    pub url: &'a str,
    pub site_info: &'a SiteInfo,
    pub fragment: Option<&'a Html>,
}

impl<'a> Scraper<'a> {
    fn get_class_text(
        selector_str: &str,
        out_of_stock_div: &str,
        fragment: &'a Html,
    ) -> Option<&'a str> {
        let selector = Selector::parse(&format!(".{} {}", selector_str, out_of_stock_div)).unwrap();
        let element = fragment.select(&selector).next().unwrap();
        let t = element.text().collect::<Vec<_>>();

        if t.is_empty() {
            return None;
        }

        Some((&t[0]).trim())
    }

    async fn in_stock(mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = reqwest::get(self.url).await?;
        let body = res.text().await?;
        let fragment = Html::parse_document(&body);
        self.fragment = Some(&fragment);

        let mut text = String::new();
        match self.site_info.stock_status_tag {
            TargetTagType::Class => {
                if let Some(t) = Scraper::get_class_text(
                    self.site_info.stock_status_selector,
                    self.site_info.out_of_stock_div,
                    &fragment,
                ) {
                    text = t.to_string();
                }
            }
            TargetTagType::ID => {
                // for node in document
                //     .find(Attr("id", self.site_info.selector_str))
                //     .descendant(Name(""))
                // {
                //     println!("{:?}", node.text());
                // }
            }
        };

        Ok(!text.contains(self.site_info.out_of_stock_text))
    }

    /// Gets the web page, and checks to see if the item is out of stock or not
    pub async fn execute(self) -> Result<bool, Box<dyn std::error::Error>> {
        self.in_stock().await
    }
}

/// The builder for the web scraper
#[derive(Copy, Clone)]
pub struct ScraperBuilder<'a> {
    url: &'a str,
    site: &'a Site,
}

impl<'a> ScraperBuilder<'a> {
    fn get_target_site(url: &'a str) -> Option<&'a Site> {
        let split = url.split('/');

        let mut site = "";
        for s in split.collect::<Vec<&str>>() {
            if s.contains(".com") {
                let domain_split = s.split('.').collect::<Vec<&str>>();
                site = domain_split[1];

                break;
            }
        }

        SITE_MAP.get(site)
    }

    /// Returns a new ScraperBuilder
    pub fn new(url: &str) -> Result<ScraperBuilder, Box<dyn std::error::Error>> {
        let site = ScraperBuilder::get_target_site(url);

        Ok(ScraperBuilder {
            url,
            site: site.unwrap(),
        })
    }

    /// Builds a new Scraper
    pub fn build(self) -> Scraper<'a> {
        let info = DIV_MAP.get(&self.site);

        Scraper {
            url: self.url,
            site_info: info.unwrap(),
            fragment: None,
        }
    }
}

/// Contains information relevant to a site
#[derive(Debug, Copy, Clone)]
pub struct SiteInfo {
    pub stock_status_tag: TargetTagType,
    pub out_of_stock_div: &'static str,
    pub stock_status_selector: &'static str,
    pub out_of_stock_text: &'static str,
    pub item_title_selector_type: TargetTagType,
    pub item_title_selector: &'static str,
}
