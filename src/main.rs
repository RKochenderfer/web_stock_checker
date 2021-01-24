mod scraper;
mod site;

use scraper::ScraperBuilder;

/*
Looking for
    <div class="product-inventory"><strong><i class="fas fa-exclamation-triangle"></i> OUT OF STOCK.</strong></div>
*/

const URLS: [&str; 4] = [
    "https://www.newegg.com/gigabyte-geforce-rtx-3090-gv-n3090aorus-x-24gd/p/N82E16814932340?Description=rtx%203090&cm_re=rtx_3090-_-14-932-340-_-Product",
    "https://www.newegg.com/gigabyte-geforce-rtx-3090-gv-n3090aorus-m-24gd/p/N82E16814932341?Description=rtx%203090&cm_re=rtx_3090-_-14-932-341-_-Product",
    "https://www.newegg.com/asus-geforce-rtx-3090-rog-strix-rtx3090-o24g-gaming/p/N82E16814126456?Description=rtx%203090&cm_re=rtx_3090-_-14-126-456-_-Product&quicklink=true",
    "https://www.newegg.com/msi-geforce-rtx-3090-rtx-3090-gaming-x-trio-24g/p/N82E16814137595?Description=rtx%203090&cm_re=rtx_3090-_-14-137-595-_-Product&quicklink=true"];

fn main() {
    let mut builders: Vec<ScraperBuilder> = Vec::new();

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

    for b in builders.iter() {
        let scraper = b.build();
    }
}
