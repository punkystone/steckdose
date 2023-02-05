use crate::{
    encryption::{decrypt, encrypt},
    errors::PlugError,
    responses::SystemInfo,
    responses::{DefaultResponse, EmeterResponse},
};
use core::time::Duration;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    vec,
};

const DEFAULT_PORT: &str = "9999";
const INFO_COMMAND: &str = "{\"system\":{\"get_sysinfo\":null}}";
const OFF_COMMAND: &str = "{\"system\":{\"set_relay_state\":{\"state\":0}}}";
const ON_COMMAND: &str = "{\"system\":{\"set_relay_state\":{\"state\":1}}}";
const EMETER_COMMAND: &str = "{\"emeter\":{\"get_realtime\":{}}}";

pub fn emeter(ip: String) -> Result<EmeterResponse, PlugError> {
    let result = &send_command(ip, EMETER_COMMAND)?;
    match serde_json::from_str::<EmeterResponse>(result) {
        Ok(json) => Ok(json),
        Err(_) => Err(PlugError::JSONError),
    }
}

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
    let mut buffer: Vec<u8> = vec![0; 4069];
    match stream.read(&mut buffer) {
        Ok(length) => {
            buffer = buffer[0..length].to_vec();
            Ok(decrypt(&mut buffer))
        }
        Err(_) => Err(PlugError::ReadError),
    }
}
