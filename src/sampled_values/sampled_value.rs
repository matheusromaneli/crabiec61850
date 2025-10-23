use byteorder::{BigEndian, ByteOrder};
use crate::sampled_values::model::{Asdu, PDUTags, SampledValue};


impl SampledValue {
    pub fn from_bytes(bytes: &[u8]) -> SampledValue {
        let no_asdu_len = bytes[11];
        let number_of_asdu: u16;
        if no_asdu_len == 1 {
            number_of_asdu = bytes[12] as u16;
        }
        else {
            number_of_asdu = BigEndian::read_u16(&bytes[12..14]);
        }
        let mut asdus: Vec<Asdu> = vec![];
        let mut asdu_start = (14 + no_asdu_len) as usize;
        for _ in 0..number_of_asdu {
            let asdu_tag = bytes[asdu_start];
            if asdu_tag != PDUTags::ASDU as u8 {
                panic!("ASDU tag is not 0x30 at offset {}", asdu_start);
            }
            let asdu_len = bytes[asdu_start + 1];
            let asdu = Asdu::from_bytes(&bytes[asdu_start + 2..asdu_start + 2 + asdu_len as usize]);
            asdus.push(asdu);
            asdu_start += 2 + asdu_len as usize;
        }
        SampledValue {
            app_id: BigEndian::read_u16(&bytes[0..2]),
            length: BigEndian::read_u16(&bytes[2..4]),
            simulation: bytes[4] >> 3 == 1,
            reserved1: [bytes[4], bytes[5]],
            reserved2: [bytes[6], bytes[7]],
            number_of_asdu: number_of_asdu,
            asdu: asdus,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        BigEndian::write_u16(&mut bytes, self.app_id);
        BigEndian::write_u16(&mut bytes, self.length);
        bytes.push((self.simulation as u8) << 3);
        bytes.push(self.reserved1[0]);
        bytes.push(self.reserved1[1]);
        bytes.push(self.reserved2[0]);
        bytes.push(self.reserved2[1]);
        bytes.push(self.asdu.len() as u8);
        for asdu in &self.asdu {
            let mut asdu_bytes = asdu.to_bytes();
            bytes.append(&mut asdu_bytes);
        }
        bytes
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

    #[test]
    fn decode_1_asdu() {
        let bytes: &[u8] = &[
            0x40, 0x02, 0x00, 0x66, 0x00, 0x00, 0x00, 0x00, // Header
            0x60, 0x5c, // PDU
            0x80, 0x01, 0x01, // number of ASDU
            0xa2, 0x57, // sequence of ASDU
            0x30, 0x55, 0x80, 0x04, 0x34, 0x30, 0x30, 0x30, 0x82, 0x02, 0x00, 0x00, 0x83, 0x04, 0x00, 0x00, 0x00,
            0x01, 0x85, 0x01, 0x01, 0x87, 0x40, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x03, 0x00, 0x00, 0x20, 0x00, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
            0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
            0xff, 0xf6, 0x00, 0x00, 0x20, 0x00 // ASDU
        ];

        let sampled_value = SampledValue::from_bytes(bytes);
        assert_eq!(0x4002, sampled_value.app_id);
        assert_eq!(102, sampled_value.length);
        assert_eq!(false, sampled_value.simulation);
        assert_eq!([0x00, 0x00], sampled_value.reserved1);
        assert_eq!([0x00, 0x00], sampled_value.reserved2);
        assert_eq!(1, sampled_value.number_of_asdu);
    }
}