#![deny(
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::perf
)]
#![allow(clippy::enum_variant_names)]
use arguments::Arguments;
use clap::Parser;

mod arguments;
mod encryption;
mod errors;
mod repository;
mod responses;
fn main() {
    let args = Arguments::parse();
    let ip = args.ip;
    let action = args.action;
    match action {
        arguments::Action::on => match repository::on(&ip) {
            Ok(response) => println!("{}", if response { "Success" } else { "Failure" }),
            Err(error) => eprintln!("{error}"),
        },
        arguments::Action::off => match repository::off(&ip) {
            Ok(response) => println!("{}", if response { "Success" } else { "Failure" }),
            Err(error) => eprintln!("{error}"),
        },
        arguments::Action::status => match repository::status(&ip) {
            Ok(response) => println!("{}", if response { "On" } else { "Off" }),
            Err(error) => eprintln!("{error}"),
        },
    }
}
