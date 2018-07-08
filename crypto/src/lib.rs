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

extern crate failure;
extern crate hex;
extern crate ring;

#[cfg(test)]
#[macro_use]
extern crate proptest;

use failure::Error;
use hex::{FromHex, ToHex};
use ring::{digest, hmac};

type Result<T> = std::result::Result<T, Error>;

pub fn bytes_to_hex_str(bytes: &[u8]) -> Result<String> {
    let mut output = String::new();
    bytes.write_hex(&mut output)?;
    Ok(output)
}

pub fn hex_str_to_bytes(hex_str: &str) -> Result<Vec<u8>> {
    Ok(Vec::from_hex(hex_str)?)
}

pub fn verify_signature(key: &[u8], msg: &[u8], signature: &[u8]) -> bool {
    let key = hmac::VerificationKey::new(&digest::SHA256, key);
    hmac::verify(&key, msg, signature)
        .map(|_| true)
        .unwrap_or(false)
}

pub fn sign(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let key = hmac::SigningKey::new(&digest::SHA256, key);
    let signature = hmac::sign(&key, msg);
    signature.as_ref().into_iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn sign_verify(key: Vec<u8>, msg: Vec<u8>) {
            let sig = sign(&key, &msg);
            assert!(verify_signature(&key, &msg, &sig));
        }
    }

    proptest! {
        #[test]
        fn to_from_hex(data: Vec<u8>) {
            assert_eq!(hex_str_to_bytes(&bytes_to_hex_str(&data).unwrap()).unwrap(), data);
        }
    }
}
