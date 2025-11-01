use std::ffi::CString;
use libc::{AF_PACKET, SO_BINDTODEVICE, SOCK_RAW, SOL_SOCKET, htons, recv, setsockopt, socket};

pub struct RawSocket {
    pub sock: libc::c_int,
}

impl RawSocket {
    pub fn new(iface: String, protocol: u16) -> Self {
        let sock = unsafe { socket(AF_PACKET, SOCK_RAW, htons(protocol) as i32) };
        
        if sock == -1 {
            panic!("Failed to create socket: {}", std::io::Error::last_os_error());
        }

        let interface_name = CString::new(iface).unwrap();

        let result = unsafe {
            setsockopt(
                sock,
                SOL_SOCKET,
                SO_BINDTODEVICE,
                interface_name.as_ptr() as *const libc::c_void,
                libc::IFNAMSIZ as libc::socklen_t,
            )
        };
        if result != 0 {
            panic!("Failed to bind socket to interface: {}", std::io::Error::last_os_error());
        }

        RawSocket { sock }
    }

    pub fn recv(&self) -> Vec<u8> {
        let mut buffer = [0u8; 65536];
        let packet_size = unsafe { recv(self.sock, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len(), 0) };
        if packet_size < 0 {
            panic!("Error receiving packet: {}", std::io::Error::last_os_error());
        }
        buffer[..packet_size as usize].to_vec()
    }


}