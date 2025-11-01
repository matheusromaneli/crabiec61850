use crate::network::socket::RawSocket;

pub fn main() {
    let socket = RawSocket::new("lo".to_string(), 0x88ba_u16);

    loop {
        let bytes = socket.recv();
        let packet = crate::network::packet::Packet::from_bytes(&bytes);
        println!("{:?}", packet);
    }
}