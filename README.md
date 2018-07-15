# BIND9 API

[![Travis Build Status](https://travis-ci.org/vbrandl/bind9-api.svg?branch=master)](https://travis-ci.org/vbrandl/bind9-api)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/vbrandl/bind9-api/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/vbrandl/bind9-api/blob/master/LICENSE-APACHE)

This is an attempt to implement an API to create, update or delete DNS records
on a BIND9 DNS server.

## Server

The server will wait for incoming requests and uses the `nsupdate` command to
perform operations on the BIND9 nameserver. For the server to work, a DNS key is
needed to perform the updates.

```
$ dnssec-keygen -r /dev/urandom -a HMAC-SHA256 -b 256 -n HOST dnskey
```

Copy the `Key` section of the resulting `Kdnskey*.private` file into a file that
looks like this:

```
key "dns-key" {
    algorithm hmac-sha256;
    secret "<your secret>";
}
```

And extend the zone section of the zones you'd like to modify in your
`named.conf.local`

```
zone "example.com" {
    type master;
    file "/var/lib/bind/db.example.com";
    ...
    allow-update { key "dns-key"; };
    ...
}
```

Now you can start the server:

```
$ ./bind9-api -k <path to dnskey> -t <your api token>
```

By default, the server will bind to `0.0.0.0:8000`. The host and port to bind
to, can be changed using the `-h` and `-p` flags respectively. For production
use, you should bind to a private IP address (LAN or VLAN) or to `127.0.0.1` and
put the server behind a reverse proxy that offers TLS.

## Client

The client is used to perform changes to the DNS zone from any server. My use
case is to perform LetsEncrypt DNS challenges. The client will look for a
configuration file in `/etc/bind9apiclient.toml` which looks like this:

```
# API server host
host = "http://127.0.0.1:8080"
# API secret
secret = "topsecret"
```

The client can perform two operations: Creating/updating and deleting DNS
records. The client is invoked like this

```
$ ./bind9-api-client -d foo.example.com -r TXT update -v foobar
$ ./bind9-api-client -d foo.example.com -r TXT delete
```

## API Description

```
POST /record
X-Api-Token: <api-token>

{
    "name": "foo.example.com",
    "value": "127.0.0.1",
    "record": "A",
    "ttl": 1337
}
```

```
DELETE /record
X-Api-Token: <api-token>

{
    "name": "foo.example.com",
    "record": "A"
}
```

The API token is a SHA256 HMAC over the request body using a pre-shared secret.

### Security Considerations

The current API design does not migrate replay attacks. An attacker that is able
to intercept a request to the API can resend the same request again to execute
the same operation.  To prevent these kinds of attacks, you should use a reverse
proxy and encrypt the connections using TLS. Future versions of the server might
provide TLS functionality by itself.

## Usage with LetsEncrypt

In `letsencrypt/`, two example scripts can be found to use the client as a
certbot hook for DNS challenges. It assumes that the client is located somewhere
in `$PATH` and that the configurations file exists.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
