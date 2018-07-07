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
extern crate serde_json;

mod cli;

use actix_web::{
    error::{self, ErrorInternalServerError, ErrorUnauthorized, JsonPayloadError, ParseError}, http,
    middleware::Logger, server, App, HttpMessage, HttpRequest, Result,
};
use data::{Delete, Update};
use failure::Error;
use futures::future::{err as FutErr, Future};
use std::{
    io::Write, process::{Command, Stdio}, sync::Arc,
};

#[derive(Debug, Fail)]
enum ExecuteError {
    #[fail(display = "Stdin error")]
    Stdin,
}

struct Config {
    token: String,
    command: String,
    key_path: String,
    ok_marker: String,
    server: String,
}

fn execute_nsupdate(input: &str, config: &Config) -> Result<String, Error> {
    info!("executing update: {}", input);
    let mut cmd = Command::new(&config.command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&["-k", &config.key_path])
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

fn delete(req: HttpRequest<Arc<Config>>) -> Box<Future<Item = &'static str, Error = error::Error>> {
    let state = req.state().clone();
    let sig = extract_signature(&req);
    if sig.is_err() {
        return Box::new(FutErr(sig.unwrap_err()));
    }
    let sig = sig.unwrap();
    let secret = state.token.clone();
    Box::new(req.body().from_err().and_then(move |body| {
        if crypto::verify_signature(secret.as_bytes(), &body, &sig) {
            let delete: Delete = serde_json::from_slice(&body)
                .map_err(|e| ErrorInternalServerError(JsonPayloadError::Deserialize(e)))?;
            info!("Deleting {} record for {}", delete.record(), delete.name());
            let stdin = format!(
                "server {}\nupdate delete {} {}\nsend\n",
                state.server,
                delete.name(),
                delete.record()
            );
            Ok(execute_nsupdate(&stdin, &state)
                .map_err(|_| ErrorInternalServerError("Error executing nsupdate"))
                .and_then(|s| {
                    if s.contains(&state.ok_marker) {
                        Ok("OK")
                    } else {
                        Err(ErrorInternalServerError("Marker not found"))
                    }
                })?)
        } else {
            Err(ErrorUnauthorized(ParseError::Header))
        }
    }))
}

fn extract_signature<S>(req: &HttpRequest<S>) -> Result<Vec<u8>> {
    Ok(req.headers()
        .get(data::TOKEN_HEADER)
        .as_ref()
        .ok_or_else(|| ErrorUnauthorized(ParseError::Header))?
        .to_str()
        .map_err(ErrorUnauthorized)
        .and_then(|s| {
            crypto::hex_str_to_bytes(s).map_err(|_| ErrorUnauthorized(ParseError::Header))
        })
        .map_err(ErrorUnauthorized)?)
}

fn update(req: HttpRequest<Arc<Config>>) -> Box<Future<Item = &'static str, Error = error::Error>> {
    let state = req.state().clone();
    let sig = extract_signature(&req);
    if sig.is_err() {
        return Box::new(FutErr(sig.unwrap_err()));
    }
    let sig = sig.unwrap();
    let secret = state.token.clone();
    Box::new(req.body().from_err().and_then(move |body| {
        if crypto::verify_signature(secret.as_bytes(), &body, &sig) {
            let update: Update = serde_json::from_slice(&body)
                .map_err(|e| ErrorInternalServerError(JsonPayloadError::Deserialize(e)))?;
            info!(
                "Updating {} record for {} with value \"{}\"",
                update.record(),
                update.name(),
                update.value()
            );
            let stdin = format!(
                "server {}\nupdate add {} {} {} {}\nsend\n",
                state.server,
                update.name(),
                update.ttl(),
                update.record(),
                update.value()
            );
            Ok(execute_nsupdate(&stdin, &state)
                .map_err(|_| ErrorInternalServerError("Error executing nsupdate"))
                .and_then(|s| {
                    if s.contains(&state.ok_marker) {
                        Ok("OK")
                    } else {
                        Err(ErrorInternalServerError("Marker not found"))
                    }
                })?)
        } else {
            Err(ErrorUnauthorized(ParseError::Header))
        }
    }))
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
    let config = Arc::new(Config {
        token,
        command,
        key_path,
        ok_marker,
        server,
    });
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
