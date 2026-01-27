use socket2::{Socket, Domain, Type, Protocol};
use std::net::TcpListener;

pub fn create_listener(addr: &str) -> std::io::Result<TcpListener> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?; // Fixes the AddrInUse error
    socket.bind(&addr.parse::<std::net::SocketAddr>().unwrap().into())?;
    socket.listen(128)?;
    Ok(socket.into())
}
