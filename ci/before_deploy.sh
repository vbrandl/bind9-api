#!/usr/bin/env bash

set -ex

main() {
    local src=$(pwd) \
            stage=$src/deploy

    mkdir -p $deploy

    cp target/x86_64-unknown-linux-musl/release/bind9-api $stage/bind9-api-${TRAVIS-TAG:1}-x86_64-musl
    cp target/x86_64-unknown-linux-musl/release/bind9-api-client $stage/bind9-api-client-${TRAVIS-TAG:1}-x86_64-musl
}

main
