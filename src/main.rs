use std::net::{IpAddr, TcpStream};

fn main() {
    let ip: IpAddr = "127.0.0.1".parse().expect("Invalid IP address");
    let start_port = 1;
    let end_port = 65535; // 65,535 max

    scan_ports(ip, start_port, end_port);
}

fn scan_port(ip: IpAddr, port: u16) -> bool {
    if let Ok(_) = TcpStream::connect((ip, port)) {
        println!("Port {} is open.", port);
        true
    } else {
        false
    }
}

fn scan_ports(ip: IpAddr, start_port: u16, end_port: u16) {
    println! {"Scanning ports in {}...", ip};
    for port in start_port..=end_port {
        scan_port(ip, port);
    }
}
