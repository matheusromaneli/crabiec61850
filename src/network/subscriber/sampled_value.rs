use crate::network::socket::RawSocket;

pub fn main() {
    let socket = RawSocket::new("lo".to_string(), 0x88b8);

    loop {
        let packet = socket.recv();
        let packet = crate::network::packet::Packet::from_bytes(&packet);
        println!("{:?}", packet);
    }
}