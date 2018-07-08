// Copyright (c) 2018 Brandl, Valentin <mail+rust@vbrandl.net>
// Author: Brandl, Valentin <mail+rust@vbrandl.net>
//
// Licensed unter the Apache License, Version 2.0 or the MIT license, at your
// option.
//
// ********************************************************************************
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// ********************************************************************************
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate actix_web;
extern crate crypto;
extern crate data;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_json;

mod cli;
mod util;

use actix_web::{
    error::{self, ErrorInternalServerError}, http, middleware::Logger, server, App, Result, State,
};
use data::{Delete, Update};
use failure::Error;
use std::{
    io::Write, process::{Command, Stdio}, sync::Arc,
};
use util::{Config, ExecuteError, Validated};

fn execute_nsupdate(input: &str, config: &Config) -> Result<String, Error> {
    info!("executing update: {}", input);
    let mut cmd = Command::new(config.command())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&["-k", config.key_path()])
        .spawn()?;
    {
        let stdin = cmd.stdin.as_mut().ok_or(ExecuteError::Stdin)?;
        stdin.write_all(input.as_bytes())?;
    }
    let output = cmd.wait_with_output()?.stdout;
    let output = String::from_utf8(output)?;
    info!("output: {}", output);
    Ok(output)
}

fn delete(
    (delete, state): (Validated<Delete>, State<Arc<Config>>),
) -> Result<&'static str, error::Error> {
    info!("Deleting {} record for {}", delete.record(), delete.name());
    let stdin = format!(
        "server {}\nupdate delete {} {}\nsend\n",
        state.server(),
        delete.name(),
        delete.record()
    );
    Ok(execute_nsupdate(&stdin, &state)
        .map_err(|_| ErrorInternalServerError("Error executing nsupdate"))
        .and_then(|s| {
            if s.contains(state.ok_marker()) {
                Ok("OK")
            } else {
                Err(ErrorInternalServerError("Marker not found"))
            }
        })?)
}

fn update(
    (update, state): (Validated<Update>, State<Arc<Config>>),
) -> Result<&'static str, error::Error> {
    info!(
        "Updating {} record for {} with value \"{}\"",
        update.record(),
        update.name(),
        update.value()
    );
    let stdin = format!(
        "server {}\nupdate add {} {} {} {}\nsend\n",
        state.server(),
        update.name(),
        update.ttl(),
        update.record(),
        update.value()
    );
    Ok(execute_nsupdate(&stdin, &state)
        .map_err(|_| ErrorInternalServerError("Error executing nsupdate"))
        .and_then(|s| {
            if s.contains(state.ok_marker()) {
                Ok("OK")
            } else {
                Err(ErrorInternalServerError("Marker not found"))
            }
        })?)
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let matches = cli::parse_args();
    let token = matches.value_of("TOKEN").unwrap().to_owned();
    let command = matches.value_of("CMD").unwrap_or("nsupdate").to_owned();
    let key_path = matches.value_of("KEYPATH").unwrap().to_owned();
    let ok_marker = matches.value_of("OKMARK").unwrap_or("").to_owned();
    let server = matches.value_of("SERVER").unwrap_or("127.0.0.1").to_owned();
    let config = Arc::new(Config::new(token, command, key_path, ok_marker, server));
    let port: u16 = matches
        .value_of("PORT")
        .unwrap_or("8000")
        .parse()
        .expect("Cannot parse port");
    let host = matches.value_of("HOST").unwrap_or("0.0.0.0");
    let host = format!("{}:{}", host, port);
    server::new(move || {
        App::with_state(config.clone())
            .middleware(Logger::default())
            .route("/record", http::Method::POST, update)
            .route("/record", http::Method::DELETE, delete)
    }).bind(host)
        .unwrap()
        .run();
}
