use std::io;
use chrono::{Datelike, NaiveDateTime, TimeZone, Utc, Weekday};
use crate::market::MarketInfo;

mod market;
mod public_web;
mod non_trade;

struct Calender {
    non_trade_cal: non_trade::NonTradeCal

}

impl Calender {
    pub fn new(non_trade_path: impl io::Read) -> Self {
        let c = non_trade::NonTradeCal::new(non_trade_path);
        Calender {
            non_trade_cal: c
        }

    }

    pub fn is_trade_day(&self, m: market::Market, cur_time: Option<NaiveDateTime>) -> bool {
        let timezone = m.time_zone();
        // 获取当前时间或使用提供的时间
        let datetime = match cur_time {
            Some(time) => timezone.from_utc_datetime(&time),
            None => timezone.from_utc_datetime(&Utc::now().naive_utc()),
        };

        // 检查日期是否为周六或周日
        let weekday = datetime.weekday();
        weekday != Weekday::Sat && weekday != Weekday::Sun
    }
}


mod test {
    #[test]
    fn test_cal() {
    }
}