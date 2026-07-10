use std::{
    fmt::Display,
    net::IpAddr,
    time::{Duration, Instant},
};

use crate::utils::duration_to_ms;

pub struct PingReply {
    pub source: IpAddr,
    pub seq: u16,
    pub ttl: u8,
    pub bytes: usize,
    pub rtt: Duration,
}

impl PingReply {
    pub fn print(&self) {
        println!(
            "{} bytes from {}: icmp_seq={} ttl={} time={:.3} ms",
            self.bytes,
            self.source,
            self.seq,
            self.ttl,
            self.rtt.as_secs_f64() * 1000.0
        );
    }
}

pub struct PingStats {
    pub destination: IpAddr,
    pub hostname: Option<String>,
    pub transmitted: u64,
    pub received: u64,
    pub rtts: Vec<Duration>,
    pub start_time: Instant,
    pub end_time: Instant,
    pub replies: Vec<PingReply>,
}

impl Display for PingStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- {} ping statistics ---", self.destination)?;

        writeln!(
            f,
            "{} packets transmitted, {} received, {:.0}% packet loss, time {}ms",
            self.transmitted,
            self.received,
            self.packet_loss(),
            self.total_time().as_millis()
        )?;

        if let (Some(min), Some(avg), Some(max), Some(mdev)) = (
            self.rtt_min(),
            self.rtt_avg(),
            self.rtt_max(),
            self.rtt_mdev(),
        ) {
            writeln!(
                f,
                "rtt min/avg/max/mdev = {:.3}/{:.3}/{:.3}/{:.3} ms",
                duration_to_ms(min),
                duration_to_ms(avg),
                duration_to_ms(max),
                duration_to_ms(mdev),
            )?;
        }

        Ok(())
    }
}

impl PingStats {
    pub fn new(dest: IpAddr, hostname: Option<String>) -> Self {
        Self {
            destination: dest,
            hostname,
            transmitted: 0,
            received: 0,
            rtts: Vec::new(),
            start_time: Instant::now(),
            end_time: Instant::now(),
            replies: Vec::new(),
        }
    }

    pub fn total_time(&self) -> Duration {
        self.end_time - self.start_time
    }

    pub fn finish(&mut self) {
        self.end_time = Instant::now();
    }

    pub fn packet_loss(&self) -> f64 {
        if self.transmitted == 0 {
            return 0.0;
        }

        let lost = self.transmitted - self.received;
        (lost as f64) * 100.0 / (self.transmitted as f64)
    }

    pub fn rtt_max(&self) -> Option<Duration> {
        self.rtts.iter().max().copied()
    }

    pub fn rtt_min(&self) -> Option<Duration> {
        self.rtts.iter().min().copied()
    }

    pub fn rtt_avg(&self) -> Option<Duration> {
        if self.rtts.is_empty() {
            return None;
        }

        Some(self.rtts.iter().sum::<Duration>() / self.rtts.len() as u32)
    }

    pub fn rtt_mdev(&self) -> Option<Duration> {
        let avg = self.rtt_avg()?;

        let variance = self
            .rtts
            .iter()
            .map(|rtt| {
                let diff = rtt.as_secs_f64() - avg.as_secs_f64();
                diff * diff
            })
            .sum::<f64>()
            / self.rtts.len() as f64;

        Some(Duration::from_secs_f64(variance.sqrt()))
    }

    pub fn record_send(&mut self) {
        self.transmitted += 1;
    }

    pub fn record_reply(&mut self, reply: PingReply) {
        self.received += 1;
        self.end_time = Instant::now();
        self.rtts.push(reply.rtt);
        self.replies.push(reply);
    }
}
