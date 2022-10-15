use crate::{
    encryption::{decrypt, encrypt},
    errors::PlugError,
    responses::DefaultResponse,
    responses::SystemInfo,
};
use core::time::Duration;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

const DEFAULT_PORT: &str = "9999";
const INFO_COMMAND: &str = "{\"system\":{\"get_sysinfo\":null}}";
const OFF_COMMAND: &str = "{\"system\":{\"set_relay_state\":{\"state\":0}}}";
const ON_COMMAND: &str = "{\"system\":{\"set_relay_state\":{\"state\":1}}}";

pub fn status(ip: String) -> Result<bool, PlugError> {
    let result = &send_command(ip, INFO_COMMAND)?;
    match serde_json::from_str::<SystemInfo>(result) {
        Ok(json) => Ok(json.system.get_sysinfo.relay_state == 1),
        Err(_) => Err(PlugError::JSONError),
    }
}

pub fn off(ip: String) -> Result<bool, PlugError> {
    let result = &send_command(ip, OFF_COMMAND)?;
    match serde_json::from_str::<DefaultResponse>(result) {
        Ok(json) => Ok(json.system.set_relay_state.err_code == 0),
        Err(_) => Err(PlugError::JSONError),
    }
}

pub fn on(ip: String) -> Result<bool, PlugError> {
    let result = &send_command(ip, ON_COMMAND)?;
    match serde_json::from_str::<DefaultResponse>(result) {
        Ok(json) => Ok(json.system.set_relay_state.err_code == 0),
        Err(_) => Err(PlugError::JSONError),
    }
}

fn send_command(ip: String, command: &str) -> Result<String, PlugError> {
    let parsed = match format!("{ip}:{DEFAULT_PORT}").parse::<SocketAddr>() {
        Ok(parsed) => parsed,
        Err(_) => return Err(PlugError::InvalidServerAddressError(ip)),
    };
    let stream = match TcpStream::connect_timeout(&parsed, Duration::new(5, 0)) {
        Ok(stream) => stream,
        Err(_) => return Err(PlugError::ServerConnectError(ip)),
    };
    let mut stream = stream;
    let encrypted = match encrypt(command) {
        Ok(encrypted) => encrypted,
        Err(error) => return Err(PlugError::EncryptionError(error)),
    };
    let result = stream.write(&encrypted);
    if result.is_err() {
        return Err(PlugError::WriteError);
    }
    let mut buffer: Vec<u8> = Vec::new();
    let result = stream.read_to_end(&mut buffer);
    if result.is_err() {
        return Err(PlugError::ReadError);
    }
    Ok(decrypt(&mut buffer))
}
