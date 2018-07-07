#[macro_use]
extern crate failure;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;

pub const TOKEN_HEADER: &str = "X-Api-Token";

header! { (XApiToken, TOKEN_HEADER) => [String] }

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Record {
    A,
    AAAA,
    PTR,
    TXT,
}

#[derive(Debug, Fail)]
pub enum ApiError {
    #[fail(display = "Parse record error")]
    ParseRecord,
    #[fail(display = "API Error")]
    RequestError,
}

impl std::str::FromStr for Record {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(Record::A),
            "AAAA" | "aaaa" => Ok(Record::AAAA),
            "TXT" | "txt" => Ok(Record::TXT),
            "PTR" | "ptr" => Ok(Record::PTR),
            _ => Err(ApiError::ParseRecord),
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Record::A => "A",
                Record::AAAA => "AAAA",
                Record::PTR => "PTR",
                Record::TXT => "TXT",
            }
        )
    }
}

#[derive(Deserialize, Serialize)]
pub struct Update {
    name: String,
    value: String,
    record: Record,
    ttl: u32,
}

impl Update {
    pub fn new(name: String, value: String, record: Record, ttl: u32) -> Self {
        Self {
            name,
            value,
            record,
            ttl: ttl,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn record(&self) -> Record {
        self.record
    }

    #[inline]
    pub fn ttl(&self) -> u32 {
        self.ttl
    }
}

#[derive(Deserialize, Serialize)]
pub struct Delete {
    name: String,
    record: Record,
}

impl Delete {
    pub fn new(name: String, record: Record) -> Self {
        Self { name, record }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn record(&self) -> Record {
        self.record
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_record() {
        assert_eq!("a".parse::<Record>().unwrap(), Record::A);
        assert_eq!("A".parse::<Record>().unwrap(), Record::A);
        assert_eq!("aaaa".parse::<Record>().unwrap(), Record::AAAA);
        assert_eq!("AAAA".parse::<Record>().unwrap(), Record::AAAA);
        assert_eq!("txt".parse::<Record>().unwrap(), Record::TXT);
        assert_eq!("TXT".parse::<Record>().unwrap(), Record::TXT);
        assert_eq!("PTR".parse::<Record>().unwrap(), Record::PTR);
        assert_eq!("ptr".parse::<Record>().unwrap(), Record::PTR);
        assert!(!"aAaA".parse::<Record>().is_ok());
    }

    #[test]
    fn record_to_str_and_parse_equals_input() {
        assert!(validate_record_parsing(Record::A));
        assert!(validate_record_parsing(Record::AAAA));
        assert!(validate_record_parsing(Record::PTR));
        assert!(validate_record_parsing(Record::TXT));
    }

    fn validate_record_parsing(record: Record) -> bool {
        format!("{}", record).parse::<Record>().unwrap() == record
    }
}
