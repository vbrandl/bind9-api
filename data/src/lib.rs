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
#![deny(missing_docs)]

//! This crate provides definitions for shared types between the client and the server

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

/// The name of the API token header: `X-Api-Token`
pub const TOKEN_HEADER: &str = "X-Api-Token";

/// Enumeration of DNS record types
#[derive(Eq, PartialEq, Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Record {
    /// A record
    A,
    /// AAAA record
    AAAA,
    /// AFSDB record
    AFSDB,
    /// APL record
    APL,
    /// CAA record
    CAA,
    /// CDNSKEY record
    CDNSKEY,
    /// CDS record
    CDS,
    /// CERT record
    CERT,
    /// CNAME record
    CNAME,
    /// DHCID record
    DHCID,
    /// DLV record
    DLV,
    /// DNAME record
    DNAME,
    /// DNSKEY record
    DNSKEY,
    /// DS record
    DS,
    /// HIP record
    HIP,
    /// IPSECKEY record
    IPSECKEY,
    /// KEY record
    KEY,
    /// KX record
    KX,
    /// LOC record
    LOC,
    /// MX record
    MX,
    /// NAPTR record
    NAPTR,
    /// NS record
    NS,
    /// NSEC record
    NSEC,
    /// NSEC3 record
    NSEC3,
    /// NSEC3PARAM record
    NSEC3PARAM,
    /// OPENPGPKEY record
    OPENPGPKEY,
    /// PTR record
    PTR,
    /// RRSIG record
    RRSIG,
    /// RP record
    RP,
    /// SIG record
    SIG,
    /// SOA record
    SOA,
    /// SRV record
    SRV,
    /// SSHFP record
    SSHFP,
    /// TA record
    TA,
    /// TKEY record
    TKEY,
    /// TLSA record
    TLSA,
    /// TSIG record
    TSIG,
    /// TXT record
    TXT,
    /// URI record
    URI,
    /// ALIAS record
    ALIAS,
}

/// Error types that the API can yield
#[derive(Debug, Fail)]
pub enum ApiError {
    /// Error while parsing a DNS record
    #[fail(display = "Parse record error")]
    ParseRecord,
    /// Error while handling a request
    #[fail(display = "API Error")]
    RequestError,
}

impl std::str::FromStr for Record {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(Record::A),
            "AAAA" | "aaaa" => Ok(Record::AAAA),
            "AFSDB" | "afsdb" => Ok(Record::AFSDB),
            "APL" | "apl" => Ok(Record::APL),
            "CAA" | "caa" => Ok(Record::CAA),
            "CDNSKEY" | "cdnskey" => Ok(Record::CDNSKEY),
            "CDS" | "cds" => Ok(Record::CDS),
            "CERT" | "cert" => Ok(Record::CERT),
            "CNAME" | "cname" => Ok(Record::CNAME),
            "DHCID" | "dhcid" => Ok(Record::DHCID),
            "DLV" | "dlv" => Ok(Record::DLV),
            "DNAME" | "dname" => Ok(Record::DNAME),
            "DNSKEY" | "dnskey" => Ok(Record::DNSKEY),
            "DS" | "ds" => Ok(Record::DS),
            "HIP" | "hip" => Ok(Record::HIP),
            "IPSECKEY" | "ipseckey" => Ok(Record::IPSECKEY),
            "KEY" | "key" => Ok(Record::KEY),
            "KX" | "kx" => Ok(Record::KX),
            "LOC" | "loc" => Ok(Record::LOC),
            "MX" | "mx" => Ok(Record::MX),
            "NAPTR" | "naptr" => Ok(Record::NAPTR),
            "NS" | "ns" => Ok(Record::NS),
            "NSEC" | "nsec" => Ok(Record::NSEC),
            "NSEC3" | "nsec3" => Ok(Record::NSEC3),
            "NSEC3PARAM" | "nsec3param" => Ok(Record::NSEC3PARAM),
            "OPENPGPKEY" | "openpgpkey" => Ok(Record::OPENPGPKEY),
            "PTR" | "ptr" => Ok(Record::PTR),
            "RRSIG" | "rrsig" => Ok(Record::RRSIG),
            "RP" | "rp" => Ok(Record::RP),
            "SIG" | "sig" => Ok(Record::SIG),
            "SOA" | "soa" => Ok(Record::SOA),
            "SRV" | "srv" => Ok(Record::SRV),
            "SSHFP" | "sshfp" => Ok(Record::SSHFP),
            "TA" | "ta" => Ok(Record::TA),
            "TKEY" | "tkey" => Ok(Record::TKEY),
            "TLSA" | "tlsa" => Ok(Record::TLSA),
            "TSIG" | "tsig" => Ok(Record::TSIG),
            "TXT" | "txt" => Ok(Record::TXT),
            "URI" | "uri" => Ok(Record::URI),
            "ALIAS" | "alias" => Ok(Record::ALIAS),
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
                Record::AFSDB => "AFSDB",
                Record::APL => "APL",
                Record::CAA => "CAA",
                Record::CDNSKEY => "CDNSKEY",
                Record::CDS => "CDS",
                Record::CERT => "CERT",
                Record::CNAME => "CNAME",
                Record::DHCID => "DHCID",
                Record::DLV => "DLV",
                Record::DNAME => "DNAME",
                Record::DNSKEY => "DNSKEY",
                Record::DS => "DS",
                Record::HIP => "HIP",
                Record::IPSECKEY => "IPSECKEY",
                Record::KEY => "KEY",
                Record::KX => "KX",
                Record::LOC => "LOC",
                Record::MX => "MX",
                Record::NAPTR => "NAPTR",
                Record::NS => "NS",
                Record::NSEC => "NSEC",
                Record::NSEC3 => "NSEC3",
                Record::NSEC3PARAM => "NSEC3PARAM",
                Record::OPENPGPKEY => "OPENPGPKEY",
                Record::PTR => "PTR",
                Record::RRSIG => "RRSIG",
                Record::RP => "RP",
                Record::SIG => "SIG",
                Record::SOA => "SOA",
                Record::SRV => "SRV",
                Record::SSHFP => "SSHFP",
                Record::TA => "TA",
                Record::TKEY => "TKEY",
                Record::TLSA => "TLSA",
                Record::TSIG => "TSIG",
                Record::TXT => "TXT",
                Record::URI => "URI",
                Record::ALIAS => "ALIAS",
            }
        )
    }
}

/// Data for an update request containing the domain name, record type,
/// record value and TTL.
#[derive(Deserialize, Serialize)]
pub struct Update {
    name: String,
    value: String,
    record: Record,
    ttl: u32,
}

impl Update {
    /// Creates a new Update object.
    pub fn new(name: String, value: String, record: Record, ttl: u32) -> Self {
        Self {
            name,
            value,
            record,
            ttl,
        }
    }

    /// Returns a reference to the name field.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the value field.
    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the record field.
    #[inline]
    pub fn record(&self) -> Record {
        self.record
    }

    /// Returns the TTL.
    #[inline]
    pub fn ttl(&self) -> u32 {
        self.ttl
    }
}

/// Data of a delete request, containing the domain name and record type.
#[derive(Deserialize, Serialize)]
pub struct Delete {
    name: String,
    record: Record,
}

impl Delete {
    /// Creates a new Delete object.
    pub fn new(name: String, record: Record) -> Self {
        Self { name, record }
    }

    /// Returns a reference to the name field.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the record type.
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
        assert!(validate_record_parsing(Record::AFSDB));
        assert!(validate_record_parsing(Record::APL));
        assert!(validate_record_parsing(Record::CAA));
        assert!(validate_record_parsing(Record::CDNSKEY));
        assert!(validate_record_parsing(Record::CDS));
        assert!(validate_record_parsing(Record::CERT));
        assert!(validate_record_parsing(Record::CNAME));
        assert!(validate_record_parsing(Record::DHCID));
        assert!(validate_record_parsing(Record::DLV));
        assert!(validate_record_parsing(Record::DNAME));
        assert!(validate_record_parsing(Record::DNSKEY));
        assert!(validate_record_parsing(Record::DS));
        assert!(validate_record_parsing(Record::HIP));
        assert!(validate_record_parsing(Record::IPSECKEY));
        assert!(validate_record_parsing(Record::KEY));
        assert!(validate_record_parsing(Record::KX));
        assert!(validate_record_parsing(Record::LOC));
        assert!(validate_record_parsing(Record::MX));
        assert!(validate_record_parsing(Record::NAPTR));
        assert!(validate_record_parsing(Record::NS));
        assert!(validate_record_parsing(Record::NSEC));
        assert!(validate_record_parsing(Record::NSEC3));
        assert!(validate_record_parsing(Record::NSEC3PARAM));
        assert!(validate_record_parsing(Record::OPENPGPKEY));
        assert!(validate_record_parsing(Record::PTR));
        assert!(validate_record_parsing(Record::RRSIG));
        assert!(validate_record_parsing(Record::RP));
        assert!(validate_record_parsing(Record::SIG));
        assert!(validate_record_parsing(Record::SOA));
        assert!(validate_record_parsing(Record::SRV));
        assert!(validate_record_parsing(Record::SSHFP));
        assert!(validate_record_parsing(Record::TA));
        assert!(validate_record_parsing(Record::TKEY));
        assert!(validate_record_parsing(Record::TLSA));
        assert!(validate_record_parsing(Record::TSIG));
        assert!(validate_record_parsing(Record::TXT));
        assert!(validate_record_parsing(Record::URI));
        assert!(validate_record_parsing(Record::ALIAS));
    }

    fn validate_record_parsing(record: Record) -> bool {
        format!("{}", record).parse::<Record>().unwrap() == record
    }
}
