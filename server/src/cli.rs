pub fn parse_args() -> ::clap::ArgMatches<'static> {
    clap_app!(api =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg TOKEN: -t --token +required +takes_value "Token to authenticate against the API")
        (@arg CMD: -c --command +takes_value "Nsupdate command (Defaults to nsupdate)")
        (@arg KEYPATH: -k --keypath +required +takes_value "Path to the DNS key")
        (@arg OKMARK: -m --marker +takes_value "Marker to detect if a operation was successful")
        (@arg PORT: -p --port +takes_value "Port to listen on (Defaults to 8000)")
        (@arg HOST: -h --host +takes_value "Host to listen on (Defaults to 0.0.0.0)")
        (@arg SERVER: -s --server +takes_value "Bind server (Defaults to 127.0.0.1)")
    ).get_matches()
}
