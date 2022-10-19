#![deny(
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::perf
)]
#![allow(clippy::enum_variant_names)]
use arguments::{Action, Arguments};
use clap::Parser;
use errors::PlugError;

mod arguments;
mod encryption;
mod errors;
mod repository;
mod responses;
fn main() {
    let args = Arguments::parse();
    let ip = args.ip;
    let action = args.action;
    match match_action(&action, ip) {
        Ok(out) => println!("{out}"),
        Err(error) => eprintln!("{error}"),
    }
}

fn match_action(action: &Action, ip: String) -> Result<&'static str, PlugError> {
    match action {
        arguments::Action::on => match repository::on(ip) {
            Ok(response) => Ok(if response { "Success" } else { "Failure" }),
            Err(error) => Err(error),
        },
        arguments::Action::off => match repository::off(ip) {
            Ok(response) => Ok(if response { "Success" } else { "Failure" }),
            Err(error) => Err(error),
        },
        arguments::Action::status => match repository::status(ip) {
            Ok(response) => Ok(if response { "On" } else { "Off" }),
            Err(error) => Err(error),
        },
    }
}
