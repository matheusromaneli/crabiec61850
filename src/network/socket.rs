use std::ffi::CString;
use libc::{AF_PACKET, ARPHRD_LOOPBACK, SO_BINDTODEVICE, SOCK_RAW, SOL_SOCKET, htons, if_nametoindex, recv, sendto, setsockopt, sockaddr_ll, socket};

pub struct RawSocket {
    pub sock: libc::c_int,
    pub iface: String,
    pub protocol: i32,
    pub sockaddr: sockaddr_ll
}

impl RawSocket {
    pub fn new(iface: String, protocol: u16) -> Self {
        let eth_p = htons(protocol) as i32;
        let sock = unsafe { socket(AF_PACKET, SOCK_RAW, eth_p) };
        
        if sock == -1 {
            panic!("Failed to create socket: {}", std::io::Error::last_os_error());
        }

        let interface_name = CString::new(iface.clone()).unwrap();

        let result = unsafe {
            setsockopt(
                sock,
                SOL_SOCKET,
                SO_BINDTODEVICE,
                interface_name.as_ptr() as *const libc::c_void,
                libc::IFNAMSIZ as libc::socklen_t,
            )
        };
        if result == -1 {
            panic!("Failed to bind socket to interface: {}", std::io::Error::last_os_error());
        }

        let if_index = unsafe { if_nametoindex(iface.as_ptr() as *const libc::c_char) };
        let sockaddr = sockaddr_ll {
            sll_family: AF_PACKET as u16,
            sll_protocol: eth_p as u16,
            sll_ifindex: if_index as i32,
            sll_hatype: ARPHRD_LOOPBACK as u16,
            sll_pkttype: 0,
            sll_halen: 6,
            sll_addr: [0u8; 8],
        };

        RawSocket { sock, iface: iface, protocol: eth_p , sockaddr: sockaddr }
    }

    pub fn recv(&self) -> Vec<u8> {
        let mut buffer = [0u8; 65536];
        let packet_size = unsafe { recv(self.sock, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len(), 0) };
        if packet_size == -1 {
            panic!("Error receiving packet: {}", std::io::Error::last_os_error());
        }
        buffer[..packet_size as usize].to_vec()
    }

    pub fn send(&self, data: &Vec<u8>) {
        let mut sockaddr = self.sockaddr.clone();
        for i in 0..6 {
            sockaddr.sll_addr[i] = data[6+i];
        }
        let result = unsafe {
            sendto(
                self.sock, data.as_ptr() as *const libc::c_void, data.len(), 0,
                &sockaddr as *const sockaddr_ll as *const libc::sockaddr,
                std::mem::size_of_val(&sockaddr) as libc::socklen_t
            )
        };
        if result == -1 {
            panic!("Error sending packet: {}", std::io::Error::last_os_error());
        }
    }


}