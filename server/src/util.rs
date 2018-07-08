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

use actix_web::{
    error::{Error, ErrorInternalServerError, ErrorUnauthorized, JsonPayloadError, ParseError},
    FromRequest, HttpMessage, HttpRequest, Result,
};
use futures::future::{err as FutErr, Future};
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Fail)]
pub enum ExecuteError {
    #[fail(display = "Stdin error")]
    Stdin,
}

pub struct Config {
    token: String,
    command: String,
    key_path: String,
    ok_marker: String,
    server: String,
}

impl Config {
    pub fn new(
        token: String,
        command: String,
        key_path: String,
        ok_marker: String,
        server: String,
    ) -> Self {
        Self {
            token,
            command,
            key_path,
            ok_marker,
            server,
        }
    }
    #[inline]
    pub fn token(&self) -> &str {
        &self.token
    }

    #[inline]
    pub fn command(&self) -> &str {
        &self.command
    }

    #[inline]
    pub fn key_path(&self) -> &str {
        &self.key_path
    }

    #[inline]
    pub fn ok_marker(&self) -> &str {
        &self.ok_marker
    }

    #[inline]
    pub fn server(&self) -> &str {
        &self.server
    }
}

pub struct Validated<T>(T);

impl<T: 'static + ::serde::de::DeserializeOwned> FromRequest<Arc<Config>> for Validated<T> {
    type Config = ();
    type Result = Box<Future<Item = Self, Error = Error>>;

    fn from_request(req: &HttpRequest<Arc<Config>>, _: &Self::Config) -> Self::Result {
        let state = req.state().clone();
        let sig = extract_signature(&req);
        if sig.is_err() {
            return Box::new(FutErr(sig.unwrap_err()));
        }
        let sig = sig.unwrap();
        Box::new(req.clone().body().from_err().and_then(move |body| {
            if ::crypto::verify_signature(state.token().as_bytes(), &body, &sig) {
                let delete: T = ::serde_json::from_slice(&body)
                    .map_err(|e| ErrorInternalServerError(JsonPayloadError::Deserialize(e)))?;
                Ok(Validated(delete))
            } else {
                Err(ErrorUnauthorized(ParseError::Header))
            }
        }))
    }
}

impl<T> Deref for Validated<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn extract_signature<S>(req: &HttpRequest<S>) -> Result<Vec<u8>> {
    Ok(req.headers()
        .get(::data::TOKEN_HEADER)
        .as_ref()
        .ok_or_else(|| ErrorUnauthorized(ParseError::Header))?
        .to_str()
        .map_err(ErrorUnauthorized)
        .and_then(|s| {
            ::crypto::hex_str_to_bytes(s).map_err(|_| ErrorUnauthorized(ParseError::Header))
        })
        .map_err(ErrorUnauthorized)?)
}
