// #![deny(warnings)]
mod mailer;
mod scraper;
mod site;

use crate::mailer::MailerBuilder;
use crate::scraper::ScraperBuilder;

/*
Looking for
    <div class="product-inventory"><strong><i class="fas fa-exclamation-triangle"></i> OUT OF STOCK.</strong></div>
*/

/// The URLS that will be searched
const URLS: [&str; 6] = [
    "https://www.newegg.com/gigabyte-geforce-rtx-3090-gv-n3090aorus-x-24gd/p/N82E16814932340?Description=rtx%203090&cm_re=rtx_3090-_-14-932-340-_-Product",
    "https://www.newegg.com/gigabyte-geforce-rtx-3090-gv-n3090aorus-m-24gd/p/N82E16814932341?Description=rtx%203090&cm_re=rtx_3090-_-14-932-341-_-Product",
    "https://www.newegg.com/asus-geforce-rtx-3090-rog-strix-rtx3090-o24g-gaming/p/N82E16814126456?Description=rtx%203090&cm_re=rtx_3090-_-14-126-456-_-Product&quicklink=true",
    "https://www.newegg.com/msi-geforce-rtx-3090-rtx-3090-gaming-x-trio-24g/p/N82E16814137595?Description=rtx%203090&cm_re=rtx_3090-_-14-137-595-_-Product&quicklink=true",
    "https://www.newegg.com/asus-geforce-rtx-3090-tuf-rtx3090-o24g-gaming/p/N82E16814126454?Description=rtx%203090&cm_re=rtx_3090-_-14-126-454-_-Product",
    "https://www.newegg.com/evga-geforce-rtx-3090-24g-p5-3987-kr/p/N82E16814487526?Description=rtx%203090&cm_re=rtx_3090-_-14-487-526-_-Product"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let mut builders: Vec<ScraperBuilder> = Vec::new();
    // Builds the scrapers
    for u in URLS.iter() {
        if let Ok(b) = ScraperBuilder::new(u) {
            builders.push(b);
        } else {
            println!(
                "There was an error while constructing the builder for {}",
                u
            );
        }
    }

    // creates a vector of sites that had inventory
    let mut found = vec![];
    for b in builders.iter() {
        let scraper = b.build();
        if scraper.execute().await? {
            found.push(scraper);
        }
    }

    if !found.is_empty() {
        // Build and send the notification
        let mb = MailerBuilder::new(found);
        let mailer = mb.build();

        match mailer.execute().await {
            Ok(res) => {
                if !res {
                    println!("Failed to send email");
                }
            }
            Err(e) => println!("Failed to send email: {:?}", e),
        }
    }

    Ok(())
}
