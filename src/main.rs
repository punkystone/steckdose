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
use util::emeter_to_table;

mod arguments;
mod encryption;
mod errors;
mod repository;
mod responses;
mod util;
fn main() {
    let args = Arguments::parse();
    let ip = args.ip;
    let action = args.action;
    match match_action(&action, ip) {
        Ok(out) => println!("{out}"),
        Err(error) => eprintln!("{error}"),
    }
}

fn match_action(action: &Action, ip: String) -> Result<String, PlugError> {
    match action {
        Action::on => match repository::on(ip) {
            Ok(response) => Ok(if response {
                "Success".to_owned()
            } else {
                "Failure".to_owned()
            }),
            Err(error) => Err(error),
        },
        Action::off => match repository::off(ip) {
            Ok(response) => Ok(if response {
                "Success".to_owned()
            } else {
                "Failure".to_owned()
            }),
            Err(error) => Err(error),
        },
        Action::status => match repository::status(ip) {
            Ok(response) => Ok(if response {
                "On".to_owned()
            } else {
                "Off".to_owned()
            }),
            Err(error) => Err(error),
        },
        Action::emeter => match repository::emeter(ip) {
            Ok(response) => return Ok(emeter_to_table(response)),
            Err(error) => Err(error),
        },
    }
}
