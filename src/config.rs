use std::{net::IpAddr, time::Duration};

use crate::error::{PingError, Result};

pub struct PingConfig {
    pub target: IpAddr,
    pub count: u64,
    pub timeout: Duration,
    pub identifier: u16,
    pub interval: Duration,
}

impl PingConfig {
    pub fn new(
        target: &str,
        count: u64,
        timeout: Duration,
        interval: Duration,
    ) -> Result<PingConfig> {
        let target: IpAddr = target
            .parse()
            .map_err(|_| PingError::AddressParse(target.to_string()))?;

        Ok(PingConfig {
            target,
            count,
            timeout,
            identifier: std::process::id() as u16,
            interval,
        })
    }
}
