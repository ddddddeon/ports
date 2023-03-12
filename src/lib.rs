use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread::{self, JoinHandle};
use std::time::Duration;

const QUARTER: u16 = u16::MAX / 4;

pub fn find_open_ports_range(addr: &str, start: u16, end: u16) {
    for n in start..end {
        let addr = IpAddr::from_str(&addr).unwrap();
        let addr = SocketAddr::new(addr, n);
        if let Ok(_) = TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
            println!("{}", n)
        }
    }
}

pub fn find_open_ports(addr: &'static str) {
    let mut threads = Vec::<JoinHandle<()>>::new();
    let mut i = 0;

    while i <= u16::MAX - QUARTER {
        threads.push(thread::spawn(move || {
            find_open_ports_range(&addr, i, i + QUARTER)
        }));
        i += QUARTER;
    }

    for t in threads {
        let _ = t.join();
    }
}
