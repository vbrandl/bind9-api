/*
 * Copyright (c) 2018 Brandl, Valentin <mail+rust@vbrandl.net>
 * Author: Brandl, Valentin <mail+rust@vbrandl.net>
 *
 * Licensed unter the Apache License, Version 2.0 or the MIT license, at your
 * option.
 *
 * ********************************************************************************
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * ********************************************************************************
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
