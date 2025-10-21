use byteorder::{BigEndian, ByteOrder};
use crate::sampled_values::model::SampledValue;


impl SampledValue {
    pub fn from_bytes(bytes: &[u8]) -> SampledValue {
        let len_asdu = bytes[11];
        let number_of_asdu: u16;
        if len_asdu == 1 {
            number_of_asdu = bytes[12] as u16;
        }
        else {
            number_of_asdu = BigEndian::read_u16(&bytes[12..14]);
        }
        SampledValue {
            app_id: BigEndian::read_u16(&bytes[0..2]),
            length: BigEndian::read_u16(&bytes[2..4]),
            simulation: bytes[4] >> 3 == 1,
            reserved1: [bytes[4], bytes[5]],
            reserved2: [bytes[6], bytes[7]],
            number_of_asdu: number_of_asdu,
            asdu: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_no_asdu() {
        let bytes: &[u8] = &[0x40,0x02,0x00,0x66,0x00,0x00,0x00,0x00,0x60,0x5c,0x80,0x01,0x00];

        let expected = SampledValue{
            app_id: 0x4002,
            length: 102,
            simulation: false,
            reserved1: [0x00, 0x00],
            reserved2: [0x00, 0x00],
            number_of_asdu: 0,
            asdu: vec![],
        };

        let sampled_value = SampledValue::from_bytes(bytes);
        assert_eq!(expected, sampled_value);
    }
}