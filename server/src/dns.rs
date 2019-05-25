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

use failure::Error;
use openssl::rsa::Rsa;
use std::{fs, str::FromStr};
use trust_dns::{
    client::{Client, ClientConnection, ClientStreamHandle, SyncClient},
    rr::{
        dnssec::{Algorithm, KeyPair, Signer}, rdata::KEY,
    },
    udp::UdpClientConnection,
};
use util::ExecuteError;

fn create_client(server: &str, pem: &str) -> Result<SyncClient<UdpClientConnection>, Error> {
    let addr = server.parse()?;
    let conn = UdpClientConnection::new(addr).map_err(|_| ExecuteError::ClientConnection)?;

    let pem = fs::read(pem)?;
    let rsa = Rsa::private_key_from_pem(&pem)?;
    let key = KeyPair::from_rsa(rsa).map_err(|_| ExecuteError::KeyPair)?;

    let sig0key = KEY::new(
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Algorithm::RSASHA256,
        key.to_public_bytes().map_err(|_| ExecuteError::KeyPair)?,
    );

    let signer = Signer::sig0(
        sig0key,
        key,
        FromStr::from_str("ddns-key").map_err(|_| ExecuteError::KeyPair)?,
    );

    Ok(SyncClient::with_signer(conn, signer))
}
