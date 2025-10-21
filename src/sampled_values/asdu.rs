use byteorder::{BigEndian, ByteOrder};
use crate::sampled_values::model::{Asdu, PhaseMeasurement, SampleSync};

impl Asdu {

    fn update<'a>(bytes: &'a [u8], _start: &mut usize, _tag: &mut u8, _len: &mut u8, _value: &mut &'a[u8]){
        if *_start >= bytes.len() {
            return;
        }
        *_tag = bytes[*_start];
        *_len = bytes[*_start + 1];
        let end = *_start + 2 + *_len as usize;
        println!("start: {}, tag: {:x}, len: {}, end: {}", _start, _tag, _len, end);
        if end < bytes.len() {
            *_value = &bytes[*_start + 2..end];
        }
        else {
            *_value = &bytes[*_start + 2..bytes.len()];
        }
        *_start = end;
    }

    pub fn from_bytes(bytes: &[u8]) -> Asdu {
        let mut start: usize = 0;
        let mut tag: u8 = 0;
        let mut len: u8 = 0;
        let mut value: &[u8] = &[];
        Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);

        let sv_id = String::from_utf8(value.to_vec()).unwrap();
        Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);

        let mut dataset: Option<String> = None;
        if tag == 0x81 {
            dataset = Some(String::from_utf8(value.to_vec()).unwrap());
            Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);
        }

        let smp_count: u16 = BigEndian::read_u16(value);
        Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);

        let conf_rev: u32 = BigEndian::read_u32(value);
        Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);

        let mut refr_tm: Option<u64> = None;
        if tag == 0x84 {
            refr_tm = Some(BigEndian::read_u64(value));
            Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);
        }

        let smp_sync: SampleSync = match value[0] {
            0 => SampleSync::Internal,
            1 => SampleSync::Local,
            2 => SampleSync::Global,
            _ => panic!("Invalid SampleSync value: {}", value[0]),
        };
        Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);

        let mut smp_rate: Option<u16> = None;
        if tag == 0x86 {
            smp_rate = Some(BigEndian::read_u16(value));
            Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);
        }

        // let measures = PhaseMeasurement::from_bytes(value);
        let measures: Vec<PhaseMeasurement> = vec![];
        Self::update(bytes, &mut start, &mut tag, &mut len, &mut value);

        let mut smp_mode: Option<u16> = None;
        if tag == 0x88 {
            smp_mode = Some(BigEndian::read_u16(value));
        }

        Asdu { 
            sv_id: sv_id,
            dataset: dataset,
            smp_count: smp_count,
            conf_rev: conf_rev,
            refr_tm: refr_tm,
            smp_sync: smp_sync,
            smp_rate: smp_rate,
            measures: measures,
            smp_mode: smp_mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_asdu_no_phs_meas() {
        let bytes: &[u8] = &[
            0x80, 0x04, 0x34, 0x30, 0x30, 0x30, 0x82, 0x02, 0x00, 0x00, 0x83, 0x04, 0x00, 0x00,
            0x00, 0x01, 0x85, 0x01, 0x01, 0x87, 0x40, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x00, 0x20, 0x00, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0xff,
            0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x00, 0xff,
            0xff, 0xff, 0xf6, 0x00, 0x00, 0x20, 0x00,
        ];

        let expected = Asdu {
            sv_id: "4000".to_string(),
            dataset: None,
            smp_count: 0,
            conf_rev: 1,
            refr_tm: None,
            smp_sync: SampleSync::Local,
            smp_rate: None,
            measures: vec![],
            smp_mode: None,
        };

        assert_eq!(expected, Asdu::from_bytes(bytes));
    }    
}