use core::fmt::Display;

pub enum EncryptionError {
    U32Error,
    U8Error,
}

pub enum PlugError {
    EncryptionError(EncryptionError),
    InvalidServerAddressError(String),
    JSONError,
    ReadError,
    WriteError,
    ServerConnectError(String),
}

impl Display for PlugError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlugError::EncryptionError(encryption_error) => match encryption_error {
                EncryptionError::U32Error => write!(f, "U32 Encryption Error"),
                EncryptionError::U8Error => write!(f, "U8 Encryption Error"),
            },
            PlugError::InvalidServerAddressError(ip) => write!(f, "Invalid Server IP: {ip}"),
            PlugError::JSONError => write!(f, "Invalid Response Data"),
            PlugError::ReadError => write!(f, "Stream Read Error"),
            PlugError::WriteError => write!(f, "Stream Write Error"),
            PlugError::ServerConnectError(ip) => write!(f, "Error connecting to {ip}"),
        }
    }
}
