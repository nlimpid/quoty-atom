use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io;

use chrono::{NaiveDate, NaiveTime, TimeZone};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;

use crate::market::{Market, MarketInfo};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct TradeDayRecord {
    market: Market,
    timezone: String,
    date: NaiveDate,
    #[serde(deserialize_with = "deserialize_trade_day_status", serialize_with = "serialize_trade_day_status")]
    status: TradeDayStatus,
}

#[derive(Debug, Eq)]
pub(crate) struct MarketTradeDay {
    market: Market,
    trade_day: NaiveDate,
}

// Implement PartialEq to enable comparisons
impl PartialEq for MarketTradeDay {
    fn eq(&self, other: &Self) -> bool {
        self.market == other.market && self.trade_day == other.trade_day
    }
}

// Implement Hash to enable use as HashMap keys
impl Hash for MarketTradeDay {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.market.market_type().hash(state);
        self.trade_day.hash(state);
    }
}


#[derive(Debug, Clone)]
enum TradeDayStatus {
    Close,
    Half(chrono::NaiveTime),
}


pub struct NonTradeCal {
    n: HashMap<MarketTradeDay, TradeDayRecord>,
}

impl NonTradeCal {
    pub fn new(path: impl io::Read) -> Self {
        let mut map: HashMap<MarketTradeDay, TradeDayRecord> = HashMap::new();


        let mut rdr = csv::Reader::from_reader(path);
        let records: Vec<TradeDayRecord> = rdr.deserialize()
            .map(|r| r.expect("Failed to deserialize"))
            .collect();

        for r in records {
            let a = r.clone();
            let key = MarketTradeDay {
                market: a.market,
                trade_day: a.date,
            };
            // Insert into the map
            map.insert(key, r);
        }


        NonTradeCal {
            n: map,
        }
    }
}


fn deserialize_trade_day_status<'de, D>(deserializer: D) -> Result<TradeDayStatus, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.starts_with("Half") {
        let time_str = s.trim_start_matches("Half(").trim_end_matches(')');
        NaiveTime::parse_from_str(time_str, "%H:%M:%S")
            .map(TradeDayStatus::Half)
            .map_err(serde::de::Error::custom)
    } else if s == "Close" {
        Ok(TradeDayStatus::Close)
    } else {
        Err(serde::de::Error::custom("Invalid status"))
    }
}

fn serialize_trade_day_status<S>(status: &TradeDayStatus, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let s = match status {
        TradeDayStatus::Close => "Close".to_string(),
        TradeDayStatus::Half(time) => format!("Half({})", time.format("%H:%M:%S")),
    };
    serializer.serialize_str(&s)
}


mod test {
    use super::*;

    #[test]
    fn test_csv() {
        let csv_data = r#"market,timezone,date,status
hk,UTC+8,2024-01-02,Close
sg,UTC+8,2024-01-03,Half(17:00:01)
"#;

        let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
        let records: Vec<TradeDayRecord> = rdr.deserialize()
            .map(|r| r.expect("Failed to deserialize"))
            .collect();

        for record in records.iter() {
            println!("{:?}", record);
        }
    }
}