use crate::network::socket::RawSocket;

pub fn main() {
    let socket = RawSocket::new("enp0s31f6".to_string(), 0x88b8_u16.to_be() as i32);

    loop {
        let packet = socket.recv();
        let packet = crate::network::packet::Packet::from_bytes(&packet);
        println!("{:?}", packet);
    }
}