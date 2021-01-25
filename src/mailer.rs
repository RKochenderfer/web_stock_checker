use crate::scraper::{Scraper, TargetTagType};
use scraper::Selector;
use sendgrid::v3::*;
use sendgrid::SendgridError;
use std::collections::HashMap;

/// Handles mailing the notification
#[derive(Debug)]
pub struct Mailer<'a> {
    urls: Vec<&'a str>,
    item_names: Vec<&'a str>,
}

impl<'a> Mailer<'a> {
    fn build_body(self) -> String {
        let mut text = String::new();

        for (i, url) in self.urls.iter().enumerate() {
            text = format!("{}<br></br><br></br>{}: {}", text, i + 1, url);
        }
        // for (url, item_name) in self.urls.iter().zip(self.item_names.iter()) {
        //     text = format!("{}\n\n{}: {}", text, item_name, url);
        // }

        text
    }

    /// Sends the email out to user
    pub async fn execute(self) -> Result<bool, SendgridError> {
        let mut cool_header = HashMap::with_capacity(2);
        cool_header.insert(String::from("x-cool"), String::from("indeed"));
        cool_header.insert(String::from("x-cooler"), String::from("cold"));

        let p =
            Personalization::new(Email::new("rayjkochenderfer@gmail.com")).add_headers(cool_header);

        let m = Message::new(Email::new("supergunner123able@gmail.com"))
            .set_subject("3090s In Stock")
            .add_content(
                Content::new()
                    .set_content_type("text/html")
                    .set_value(&self.build_body()),
            )
            .add_personalization(p);

        let mut env_vars = ::std::env::vars();
        let api_key = env_vars.find(|v| v.0 == "SG_API_KEY").unwrap();
        let sender = Sender::new(api_key.1);
        let resp = sender.send(&m).await?;
        println!("status: {}", resp.status());

        Ok(true)
    }
}

/// Handles building the Mailer
#[derive(Clone)]
pub struct MailerBuilder<'a> {
    scrapers: Vec<Scraper<'a>>,
}

impl<'a> MailerBuilder<'a> {
    fn get_class_text(scraper: Scraper<'a>) -> Option<&'a str> {
        let selector =
            Selector::parse(&format!(".{}", scraper.site_info.item_title_selector)).unwrap();
        if let Some(fragment) = scraper.fragment {
            if let Some(found) = fragment.select(&selector).next() {
                let t = found.text().collect::<Vec<_>>();

                if t.is_empty() {
                    return None;
                }

                return Some((&t[0]).trim());
            }
        }

        None
    }

    fn get_item_names(scraper: Scraper<'a>) -> &'a str {
        // for f in self.fragments {}
        let mut text = "";
        match scraper.site_info.item_title_selector_type {
            TargetTagType::Class => {
                if let Some(t) = MailerBuilder::get_class_text(scraper) {
                    text = t
                }
            }
            TargetTagType::ID => {}
        }

        text
    }

    /// Creates a new MailBuilder
    pub fn new(scrapers: Vec<Scraper<'a>>) -> MailerBuilder<'a> {
        MailerBuilder {
            scrapers: scrapers.clone(),
        }
    }

    /// Builds the mailer
    pub fn build(self) -> Mailer<'a> {
        // let iter = self.urls.iter().zip(self.fragments.iter());

        let mut urls = vec![];
        let mut item_names = vec![];
        for s in self.scrapers {
            urls.push(s.url);
            item_names.push(MailerBuilder::get_item_names(s));
        }

        Mailer { urls, item_names }
    }
}
