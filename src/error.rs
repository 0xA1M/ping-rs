use std::fmt::Display;

#[derive(Debug)]
pub enum PingError {
    PermissionDenied(std::io::Error),
    SocketFailed(std::io::Error),
    AddressParse(String),
    DnsFailed(String),
    IcmpProtocol(String),
    Timeout,
}

pub type Result<T> = std::result::Result<T, PingError>;

impl Display for PingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PingError::PermissionDenied(_) => {
                write!(f, "permission denied: root or CAP_NET_RAW needed")
            }
            PingError::SocketFailed(err) => write!(f, "socket failed: {}", err),
            PingError::AddressParse(input) => write!(f, "invalid address '{}'", input),
            PingError::DnsFailed(hostname) => write!(f, "failed to resolve '{}'", hostname),
            PingError::IcmpProtocol(detail) => write!(f, "bad ICMP response: {}", detail),
            PingError::Timeout => write!(f, "request timed out"),
        }
    }
}

impl std::error::Error for PingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PingError::PermissionDenied(err) | PingError::SocketFailed(err) => Some(err),
            _ => None,
        }
    }
}
