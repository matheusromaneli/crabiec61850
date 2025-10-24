use crate::protocols::ethernet::model::Ethernet;

impl Ethernet {
    pub fn from_bytes(bytes: &[u8]) -> Ethernet {
        Ethernet {
            dst_mac: [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]],
            src_mac: [bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11]],
            ether_type: [bytes[12], bytes[13]],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.append(&mut self.dst_mac.to_vec());
        bytes.append(&mut self.src_mac.to_vec());
        bytes.append(&mut self.ether_type.to_vec());
        bytes
    }
}