use std::net::TcpStream;

fn main() {
    let target_ip = "127.0.0.1";
    let start_port = 1;
    let end_point = 100;

    for port in start_port..=end_point {
        if is_port_open(&target_ip, port) {
            println!("Port {} is open", port);
        }
    }
}

fn is_port_open(ip: &str, port: u16) -> bool {
    if let Ok(stream) = TcpStream::connect((ip, port)) {
        let _ = stream.shutdown(std::net::Shutdown::Both);
        true
    } else {
        false
    }
}
