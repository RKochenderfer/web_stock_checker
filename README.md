# web_stock_cheker
A simple web scraper that checks a web page to see if an item is in stock. If it is found, it sends out an email using
sendgrid to notify a given user.

## Important Notes
The code checks for a env variable `SG_API_KEY` that must have a sendgrid api key