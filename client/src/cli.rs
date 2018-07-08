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

pub fn parse_cli() -> ::clap::ArgMatches<'static> {
    clap_app!(bind9_api_client =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg CONFIG: -c --config +takes_value "Path to config file (Defaults to /etc/bind9apiclient.toml)")
        (@arg DOMAIN: -d --domain +takes_value +required "Domain to create")
        (@arg RECORD: -r --record +takes_value "The record type (Defaults to TXT)")
        (@subcommand update =>
            (about: "Creates a new record")
            (@arg VALUE: -v --value +takes_value +required "Value to write in the record")
            (@arg TTL: -t --ttl + takes_value "TTL of the record (Defaults to 8640)")
        )
        (@subcommand delete =>
            (about: "Deletes a record")
        )
    ).get_matches()
}
