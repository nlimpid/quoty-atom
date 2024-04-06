use std::time::Duration;

use chrono;
use reqwest::header;
use snafu::prelude::*;

struct Nasdaq {}

impl Nasdaq {}

struct NonTradeDay {
    date: chrono::naive::NaiveDate,
    time_zone: String,
}

enum MarketStatus {
    Close,
    HalfOpen(chrono::naive::NaiveTime),
}

#[derive(Snafu, Debug)]
enum CrawlerError {
    #[snafu(display("{message}"))]
    H { message: String, source: reqwest::Error },

    #[snafu(whatever, display("{message}"))]
    Generic {
        message: String,
        #[snafu(source(from(Box < dyn std::error::Error >, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
}


fn get_holiday() -> Result<Vec<NonTradeDay>, CrawlerError> {
    let res: Vec<NonTradeDay> = vec![];
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7,zh-TW;q=0.6,ja;q=0.5".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert(header::COOKIE, "leadChannelMostRecent=Search; recentlyViewedList=LVMHF|Stocks,ADXDF|Stocks,ADEX|Stocks,ABC|Stocks,NDAQ|Stocks,VLTO$|Stocks,UK|stocks,UKOMW|stocks,IXBK|Index,QQQ|ETF,GE|Stocks; OptanonAlertBoxClosed=2024-04-03T07:43:34.158Z; OTGPPConsent=DBABLA~BVQqAAAACgA.QA; visid_incap_2170999=8sf0yJ4ASh2RvD6Dh3We8duyD2YAAAAAQUIPAAAAAABh8b98JhC0THk8RPfFzb1m; nlbi_2170999=RcQsIO3mBw6UZkoxiMYI0AAAAABUmJPpAy2PElvbw5mggT0q; incap_ses_797_2170999=vRMcGyuyS2//Q0wFeIQPC9uyD2YAAAAAR3NdTkq1C0uvq4QlKrVobg==; tr_jt_okta=; nasdaq-mnop=true; ak_bmsc=5463D618EC6CD948CB92EB5CE502042E~000000000000000000000000000000~YAAQLtgjFydZbIeOAQAACGFVrReoprzN25uHSfXJOpqHgrEDa6IK+GKuuML6t433pL6dr7K2fooiAVbOlt3sIFHhDh5kCaFLJcLRTsW15Ilkto0Z41klLNrS1/dK9xMWCJbeA5ZBDJdwqbecwg17L2q37W5BFaCvQJkA63ByyT5YgJAvRVCXd6EUw95CG4DtP5xDgy8DjO/nq+9LKDiqzb4Iu90GmaQcoX4A+nb7fXV00UxL3FEkPMQ1T4rNYey/pOSiEdlIEXxxXM2ZfXqrIkmVKr18uDmtEFqGcIrVNdCWALsjz7w0M7ZVwFWMTVrh7ggBDfhJnBvVTCxMX9bx2bFJwaXC6DDsHbLj2eEN21NBIB8Q7fjyAhoBarEW0NQQ1ExRMSF70t4/; volatileMarket=true; bm_mi=F6FBD71A0C23FB7299CB4F9B3103923E~YAAQLtgjF+j/bYeOAQAAIoaCrRfyzwFjS52+8B90wkRCAolcI2mRdWxijcZYL5t1vNnYmg0QKGggEKKzCxj2jzhdY4I5bycshlTBkkQVKRfjukQroBIRtMhctu8IBXheArO5tgu5AQ/DdftDU21cBUIo51hFcRQYxMADeOP/ruA1s5c9RjeanNFsKs7dhpny5pPZB6RQU8+B3PPWmwFNua1KmALNyMS0QPgMwtFKnR+xhRWDLGwZ/McLiK4aULmJvDO16laQ4Ty93REn0WridQ+P3YCEu+hMo8ms/pZy5IqlrhmtLYtUfmLfrvu9kpifsZiueh/g6CUYvdVgWsERsT7ovWntVBkD1fVo9t5KgBRY7c2/lglufu3w~1; bm_sv=8285AD2262281D3D7BB71A077ADC6E2B~YAAQLtgjF+n/bYeOAQAAIoaCrRcviqd8VoBpi5d2ACGqXa2aORG30Mu3PmUizBNlDTsw42BFgau7JIHm927snIiW0ge9Z13/KO4ubUb6h/gjAzDh2JwfTaopC2vl8P1QVG6apgs+ZyzW1TulcRKe3hab3c9Ov9+DUEwFa9L7OoxfBYm6TptM44vI48l+OU2NZKkQXvrX6CxEWVTsbsDrgkuZkM9vLONfa4vZCZjtqEhrZ5oalVua+CTO3pvu4DBn~1; entryUrl=https://www.nasdaq.com/market-activity/stock-market-holiday-schedule; entryReferringURL=; OptanonConsent=isGpcEnabled=0&datestamp=Fri+Apr+05+2024+17%3A06%3A36+GMT%2B0800+(China+Standard+Time)&version=202402.1.0&browserGpcFlag=0&isIABGlobal=false&hosts=&landingPath=NotLandingPage&groups=C0003%3A1%2CC0001%3A1%2CC0004%3A1%2CC0002%3A1%2COSSTA_BG%3A1&AwaitingReconsent=false&GPPCookiesCount=1&geolocation=HK%3B".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("pragma", "no-cache".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Google Chrome\";v=\"123\", \"Not:A-Brand\";v=\"8\", \"Chromium\";v=\"123\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36".parse().unwrap());


    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Some(Duration::new(10, 0)))
        .build()
        .unwrap();
    let text = client.get("https://www.nasdaq.com/market-activity/stock-market-holiday-schedule")
        .headers(headers)
        .send().context(HSnafu { message: "send" })?
        .text().context(HSnafu { message: "text" })?;
    let doc = scraper::Html::parse_document(&text);

    let selector = scraper::Selector::parse("@nsdq-l-grid").unwrap();
    for element in doc.select(&selector) {
        println!("element is {}", element.value().name());
    }
    doc.select(&selector);


    Ok(res)
}

struct nasdaqHoliday {
    date: String,
    name: String,
    status: String,
}

// https://www.nasdaqtrader.com/trader.aspx?id=calendar
fn get_nasdaqtrader_holiday() -> Result<Vec<NonTradeDay>, CrawlerError> {
    let res: Vec<NonTradeDay> = vec![];
    let mut headers = header::HeaderMap::new();
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Some(Duration::new(10, 0)))
        .build()
        .unwrap();
    let text = client.get("https://www.nasdaqtrader.com/trader.aspx?id=calendar")
        .headers(headers)
        .send().context(HSnafu { message: "send" })?
        .text().context(HSnafu { message: "text" })?;
    let doc = scraper::Html::parse_document(&text);

    let cell_selector = scraper::Selector::parse("td").unwrap();

    let selector = scraper::Selector::parse(".dataTable > table > tbody:nth-of-type(2) > tr").unwrap();
    let results: Vec<nasdaqHoliday> = doc
        .select(&selector)
        .map(|element| {
            let mut tds = element.select(&cell_selector);
            nasdaqHoliday {
                date: tds.next().map(|e| e.inner_html()).unwrap_or_default(),
                name: tds.next().map(|e| e.inner_html()).unwrap_or_default(),
                status: tds.next().map(|e| e.inner_html()).unwrap_or_default(),
            }
        })
        .collect();

    for result in results {
        println!("Date: {}, Name: {}, Status: {}", result.date, result.name, result.status);
    }


    Ok(res)
}


mod test {
    use super::*;

    #[test]
    fn get_data() {
        let got = get_holiday();
        match got {
            Ok(_) => {}
            Err(e) => { println!("error is {:?}", e) }
        }
    }


    #[test]
    fn get_get_nasdaqtrader_holiday() {
        let got = get_nasdaqtrader_holiday();
        match got {
            Ok(_) => {}
            Err(e) => { println!("error is {:?}", e) }
        }
    }
}

