use crate::site::Site;
use lazy_static::lazy_static;
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
            target_div: r#"<div class="product-inventory"></div>"#,
            out_of_stock_div: r#"<strong><i class="fas fa-exclamation-triangle"></i> OUT OF STOCK.</strong>"#,
        });

        m
    };
}

/// The web scraper
#[derive(Debug)]
pub struct Scraper<'a> {
    url: &'a str,
    site_info: &'a SiteInfo,
}

/// The builder for the web scraper
#[derive(Copy, Clone)]
pub struct ScraperBuilder<'a> {
    url: &'a str,
    site: &'a Site,
}

impl<'a> ScraperBuilder<'a> {
    fn get_target_site(url: &'a str) -> Option<&'a Site> {
        let split = url.split(r#"/"#);

        let mut site = "";
        for s in split.collect::<Vec<&str>>() {
            if s.contains(".com") {
                let domain_split = s.split(r#"."#).collect::<Vec<&str>>();
                site = domain_split[1];

                break;
            }
        }

        println!("{}", site);
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
        }
    }
}

#[derive(Debug)]
struct SiteInfo {
    pub target_div: &'static str,
    pub out_of_stock_div: &'static str,
}
