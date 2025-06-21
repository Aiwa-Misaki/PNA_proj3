use std::net::SocketAddr;

pub fn validate_address(addr: &str) -> Result<SocketAddr, String> {
    let socket: SocketAddr = addr.parse().expect("invalid ip address");
    Ok(socket)
}
