use crate::protocols::{ethernet::model::Ethernet, sampled_values::model::SampledValue};

#[derive(PartialEq, Debug)]
pub struct Packet {
    pub ether_type: [u8; 2],
    pub ethernet: Ethernet,
    pub sampled_value: SampledValue,
}

impl Packet {
    pub fn from_bytes(bytes: &[u8]) -> Packet {
        let eth_model = Ethernet::from_bytes(&bytes[0..14]);
        let sv_model = SampledValue::from_bytes(&bytes[14..]);
        let last_eth_type = eth_model.ether_type;
        Packet {
            ether_type: last_eth_type,
            ethernet: eth_model,
            sampled_value: sv_model,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.append(&mut self.ethernet.to_bytes());
        bytes.append(&mut self.sampled_value.to_bytes());
        bytes
    }
}