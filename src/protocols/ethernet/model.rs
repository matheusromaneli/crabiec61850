#[derive(PartialEq, Debug)]
pub struct Ethernet {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ether_type: [u8; 2],
}
