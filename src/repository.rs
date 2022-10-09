use std::{
    error::Error,
    io::{Read, Write},
    net::{AddrParseError, SocketAddr, TcpStream},
    time::Duration,
};

use crate::errors::{
    invalid_server_address_error::InvalidServerAddressError, json_error::JSONError,
    read_error::ReadError, write_error::WriteError,
};
use crate::{
    encryption::{decrypt, encrypt},
    errors::encryption_error::EncryptionError,
    responses::SystemInfo,
};
use crate::{errors::server_connect_error::ServerConnectError, responses::DefaultResponse};

const DEFAULT_PORT: &str = "9999";
const INFO_COMMAND: &str = "{\"system\":{\"get_sysinfo\":null}}";
const OFF_COMMAND: &str = "{\"system\":{\"set_relay_state\":{\"state\":0}}}";
const ON_COMMAND: &str = "{\"system\":{\"set_relay_state\":{\"state\":1}}}";

pub fn status(ip: &String) -> Result<bool, Box<dyn Error>> {
    let result = &send_command(ip, INFO_COMMAND)?;
    let json: Result<SystemInfo, serde_json::Error> = serde_json::from_str(result);
    if json.is_err() {
        return Err(Box::new(JSONError));
    }
    Ok(json.unwrap().system.get_sysinfo.relay_state == 1)
}

pub fn off(ip: &String) -> Result<bool, Box<dyn Error>> {
    let result = &send_command(ip, OFF_COMMAND)?;
    let json: Result<DefaultResponse, serde_json::Error> = serde_json::from_str(result);
    if json.is_err() {
        return Err(Box::new(JSONError));
    }
    Ok(json.unwrap().system.set_relay_state.err_code == 0)
}

pub fn on(ip: &String) -> Result<bool, Box<dyn Error>> {
    let result = &send_command(ip, ON_COMMAND)?;
    let json: Result<DefaultResponse, serde_json::Error> = serde_json::from_str(result);
    if json.is_err() {
        return Err(Box::new(JSONError));
    }
    Ok(json.unwrap().system.set_relay_state.err_code == 0)
}

fn send_command(ip: &String, command: &str) -> Result<String, Box<dyn Error>> {
    let parsed: Result<SocketAddr, AddrParseError> = format!("{ip}:{DEFAULT_PORT}").parse();
    if parsed.is_err() {
        return Err(Box::new(InvalidServerAddressError { ip: ip.to_string() }));
    }
    let stream = TcpStream::connect_timeout(&parsed.unwrap(), Duration::new(5, 0));
    if stream.is_err() {
        return Err(Box::new(ServerConnectError { ip: ip.to_string() }));
    }
    let mut stream = stream.unwrap();
    let encrypted = encrypt(command);
    if encrypted.is_err() {
        return Err(Box::new(EncryptionError));
    }
    let result = stream.write(&encrypted.unwrap());
    if result.is_err() {
        return Err(Box::new(WriteError));
    }
    let mut buffer: Vec<u8> = Vec::new();
    let result = stream.read_to_end(&mut buffer);
    if result.is_err() {
        return Err(Box::new(ReadError));
    }
    Ok(decrypt(&mut buffer))
}
