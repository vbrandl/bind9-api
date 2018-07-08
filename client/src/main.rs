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
extern crate clap;
extern crate crypto;
extern crate data;
extern crate failure;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate openssl_probe;
extern crate pretty_env_logger;
extern crate reqwest;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cli;

use failure::Error;

use data::{ApiError, Delete, Record, Update};

use std::borrow::Cow;

type Result<T> = std::result::Result<T, Error>;

header! { (XApiToken, data::TOKEN_HEADER) => [String] }

#[derive(Eq, PartialEq, Clone, Copy)]
enum Method {
    POST,
    DELETE,
}

#[derive(Deserialize)]
struct Config<'a> {
    #[serde(borrow)]
    host: Cow<'a, str>,
    #[serde(borrow)]
    secret: Cow<'a, str>,
}

fn delete(config: &Config, record: Record, domain: &str) -> Result<()> {
    let delete = Delete::new(domain.to_owned(), record);
    let res = call_api(config, delete, Method::DELETE)?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::RequestError.into())
    }
}

fn update(config: &Config, record: Record, domain: &str, value: &str, ttl: u32) -> Result<()> {
    let update = Update::new(domain.to_owned(), value.to_owned(), record, ttl);
    let res = call_api(config, update, Method::POST)?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::RequestError.into())
    }
}

fn call_api<D: serde::Serialize>(
    config: &Config,
    data: D,
    method: Method,
) -> Result<reqwest::Response> {
    let data_s = serde_json::to_string(&data)?;
    info!("body: {}", data_s);
    let signature = crypto::sign(config.secret.as_bytes(), data_s.as_bytes());
    let signature = crypto::bytes_to_hex_str(&signature)?;
    let client = reqwest::Client::new();
    let url = format!("{}/record", config.host);
    Ok(if method == Method::POST {
        client.post(&url)
    } else {
        client.delete(&url)
    }.header(XApiToken(signature))
        .json(&data)
        .send()?)
}

fn main() -> Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let matches = cli::parse_cli();
    let record: Record = matches
        .value_of("RECORD")
        .unwrap_or("TXT")
        .parse()
        .expect("Invalid record type");
    let domain = matches.value_of("DOMAIN").unwrap();
    let config_path = matches
        .value_of("CONFIG")
        .unwrap_or("/etc/bind9apiclient.toml");
    let config = std::fs::read_to_string(config_path).expect("Cannot read config file");
    let config: Config = toml::from_str(&config).expect("Cannot parse config file");
    if let Some(matches) = matches.subcommand_matches("update") {
        let ttl = matches
            .value_of("TTL")
            .unwrap_or("8640")
            .parse()
            .expect("Cannot parse TTL");
        update(
            &config,
            record,
            domain,
            matches.value_of("VALUE").unwrap(),
            ttl,
        )?;
    } else if matches.subcommand_matches("delete").is_some() {
        delete(&config, record, domain)?;
    }
    Ok(())
}
