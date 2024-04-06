use std::fmt;
use std::str::FromStr;

use chrono_tz::Tz;
use iso_country::Country;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;

// 市场信息特征
pub trait MarketInfo {
    fn market_type(&self) -> &str;
    fn country_code(&self) -> Country;
    fn time_zone(&self) -> Tz;
}

// US Market
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct USMarket;

impl MarketInfo for USMarket {
    fn market_type(&self) -> &str {
        "US"
    }

    fn country_code(&self) -> Country {
        Country::US
    }

    fn time_zone(&self) -> Tz {
        "America/New_York".parse().unwrap()
    }
}

// HK Market
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HKMarket;

impl MarketInfo for HKMarket {
    fn market_type(&self) -> &str {
        "HK"
    }

    fn country_code(&self) -> Country {
        Country::HK
    }

    fn time_zone(&self) -> Tz {
        "Asia/Hong_Kong".parse().unwrap()
    }
}


// CN Market
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CNMarket;

impl MarketInfo for CNMarket {
    fn market_type(&self) -> &str {
        "CN"
    }
    fn country_code(&self) -> Country {
        Country::CN
    }
    fn time_zone(&self) -> Tz {
        "Asia/Shanghai".parse().unwrap()
    }
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SGMarket;

impl MarketInfo for SGMarket {
    fn market_type(&self) -> &str {
        "SG"
    }
    fn country_code(&self) -> Country {
        Country::SG
    }
    fn time_zone(&self) -> Tz {
        "Asia/Singapore".parse().unwrap()
    }
}

// 市场枚举
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Market {
    US(USMarket),
    HK(HKMarket),
    CN(CNMarket),
    SG(SGMarket),
}

impl FromStr for Market {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "US" => Ok(Market::US(USMarket)),
            "HK" => Ok(Market::HK(HKMarket)),
            "CN" => Ok(Market::CN(CNMarket)),
            "SG" => Ok(Market::SG(SGMarket)),
            _ => Err(()),
        }
    }
}

impl Market {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "US" => Some(Market::US(USMarket {})),
            "HK" => Some(Market::HK(HKMarket {})),
            "CN" => Some(Market::CN(CNMarket {})),
            "SG" => Some(Market::SG(SGMarket {})),
            _ => None,
        }
    }
}


macro_rules! impl_market_info_for_market {
    ($($variant:ident($type:ty)),* $(,)?) => {
        impl MarketInfo for Market {
            fn market_type(&self) -> &str {
                match self {
                    $(Market::$variant(market) => market.market_type(),)*
                }
            }
            fn country_code(&self) -> Country {
                match self {
                    $(Market::$variant(market) => market.country_code(),)*
                }
            }
            fn time_zone(&self) -> Tz {
                match self {
                    $(Market::$variant(market) => market.time_zone(),)*
                }
            }
        }
    };
}

impl_market_info_for_market! {
    US(USMarket),
    HK(HKMarket),
    CN(CNMarket),
    SG(SGMarket),
}

// Custom Serialization
impl Serialize for Market {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match *self {
            Market::US(_) => serializer.serialize_str("US"),
            Market::HK(_) => serializer.serialize_str("HK"),
            Market::CN(_) => serializer.serialize_str("CN"),
            Market::SG(_) => serializer.serialize_str("SG"),
        }
    }
}

impl<'de> Deserialize<'de> for Market {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct StringVisitor;

        impl<'de> de::Visitor<'de> for StringVisitor {
            type Value = Market;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a market")
            }

            fn visit_str<E>(self, value: &str) -> Result<Market, E>
                where
                    E: de::Error,
            {
                value.parse::<Market>().map_err(|_| E::custom(format!("unknown market {}", value)))
            }
        }

        deserializer.deserialize_str(StringVisitor)
    }
}



mod test {
    use super::*;

    #[test]
    fn test_market() {
        let market_type = "us";
        if let Some(market) = Market::from_str(market_type) {
            println!("market is {}", market.time_zone())
        }
    }
}
