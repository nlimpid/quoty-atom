use std::fs::File;
use std::io::BufReader;
use snafu::prelude::*;

pub struct HKEX {}


#[derive(Snafu, Debug)]
enum Error {
    #[snafu(whatever, display("{message}"))]
    Generic {
        message: String,
        #[snafu(source(from(Box < dyn std::error::Error >, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;


impl HKEX {
    fn new() -> Self {
        HKEX {}
    }

    pub fn get_calender(&self) -> Result<()> {
        let buf = BufReader::new(File::open("/Users/nlimpid/Downloads/hkex-calendar-xxx.ics")
            .unwrap());

        let reader = ical::IcalParser::new(buf);

        for line in reader {
            println!("{:?}", line.unwrap());
        }
        Ok(())
    }
}

mod test {
    use super::*;

    #[test]
    fn test_cal() -> Result<()> {
        let l = HKEX::new();
        l.get_calender()
    }
}
