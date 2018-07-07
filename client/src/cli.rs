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
