use byteorder::{BigEndian, ByteOrder};
use crate::sampled_values::model::{ASDUTags, Asdu, Phases, SampleSync};

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

        let measures = Phases::from_bytes(value);
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

    fn sv_id_to_bytes(bytes: &mut Vec<u8>, sv_id: &str) {
        bytes.push(ASDUTags::SvId as u8);
        let sv_id_bytes = sv_id.as_bytes();
        bytes.push(sv_id_bytes.len() as u8);
        bytes.append(&mut sv_id_bytes.to_vec());
    }

    fn dataset_to_bytes(bytes: &mut Vec<u8>, dataset: &str) {
        bytes.push(ASDUTags::Dataset as u8);
        let dataset_bytes = dataset.as_bytes();
        bytes.push(dataset_bytes.len() as u8);
        bytes.append(&mut dataset_bytes.to_vec());
    }

    fn smp_count_to_bytes(bytes: &mut Vec<u8>, smp_count: u16) {
        bytes.push(ASDUTags::SmpCount as u8);
        bytes.push(2);
        bytes.append(&mut vec![0; 2]);
        BigEndian::write_u16(&mut bytes[2..], smp_count);
    }

    fn conf_rev_to_bytes(bytes: &mut Vec<u8>, conf_rev: u32) {
        bytes.push(ASDUTags::ConfRev as u8);
        bytes.push(4);
        bytes.append(&mut vec![0; 4]);
        BigEndian::write_u32(&mut bytes[2..], conf_rev);
    }

    fn refr_tm_to_bytes(bytes: &mut Vec<u8>, refr_tm: u64) {
        bytes.push(ASDUTags::RefrTm as u8);
        bytes.push(8);
        bytes.append(&mut vec![0; 8]);
        BigEndian::write_u64(&mut bytes[2..], refr_tm);
    }

    fn smp_sync_to_bytes(bytes: &mut Vec<u8>, smp_sync: u8) {
        bytes.push(ASDUTags::SmpSync as u8);
        bytes.push(1);
        bytes.push(smp_sync);
    }

    fn smp_rate_to_bytes(bytes: &mut Vec<u8>, smp_rate: u16) {
        bytes.push(ASDUTags::SmpRate as u8);
        bytes.push(2);
        BigEndian::write_u16(bytes, smp_rate);
    }

    fn measures_to_bytes(bytes: &mut Vec<u8>, measures: &Phases) {
        bytes.push(ASDUTags::Measures as u8);
        bytes.push(64);
        bytes.append(&mut measures.to_bytes());
    }

    fn smp_mode_to_bytes(bytes: &mut Vec<u8>, smp_mode: u16) {
        bytes.push(ASDUTags::SmpMode as u8);
        bytes.push(2);
        BigEndian::write_u16(bytes, smp_mode);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        Self::sv_id_to_bytes(&mut bytes, &self.sv_id);

        if let Some(dataset) = &self.dataset {
            Self::dataset_to_bytes(&mut bytes, dataset);
        }

        Self::smp_count_to_bytes(&mut bytes, self.smp_count);

        Self::conf_rev_to_bytes(&mut bytes, self.conf_rev);

        if let Some(refr_tm) = &self.refr_tm {
            Self::refr_tm_to_bytes(&mut bytes, *refr_tm);
        }

        Self::smp_sync_to_bytes(&mut bytes, self.smp_sync as u8);

        if let Some(smp_rate) = &self.smp_rate {
            Self::smp_rate_to_bytes(&mut bytes, *smp_rate);
        }

        Self::measures_to_bytes(&mut bytes, &self.measures);

        if let Some(smp_mode) = &self.smp_mode {
            Self::smp_mode_to_bytes(&mut bytes, *smp_mode);
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::sampled_values::model::{PhaseMeasurement, PhaseMeasures, Phases};

    use super::*;

    #[test]
    fn decode_asdu() {
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
            measures: Phases::from_bytes(&bytes[21..21+64]),
            smp_mode: None,
        };

        assert_eq!(expected, Asdu::from_bytes(bytes));
    }

    #[test]
    fn add_sv_id() {
        let mut bytes: Vec<u8> = vec![];
        Asdu::sv_id_to_bytes(&mut bytes, "4000");
        assert_eq!(bytes, vec![0x80, 0x04, 0x34, 0x30, 0x30, 0x30]);
    }

    #[test]
    fn add_smp_count() {
        let mut bytes: Vec<u8> = vec![];
        Asdu::smp_count_to_bytes(&mut bytes, 0);
        assert_eq!(bytes, vec![0x82, 0x02, 0x00, 0x00]);
    }
    
    #[test]
    fn add_conf_rev() {
        let mut bytes: Vec<u8> = vec![];
        Asdu::conf_rev_to_bytes(&mut bytes, 1);
        assert_eq!(bytes, vec![0x83, 0x04, 0x00, 0x00, 0x00, 0x01]);
    }

    #[test]
    fn add_smp_sync() {
        let mut bytes: Vec<u8> = vec![];
        Asdu::smp_sync_to_bytes(&mut bytes, SampleSync::Local as u8);
        assert_eq!(bytes, vec![0x85, 0x01, 0x01]);
    }

    #[test]
    fn add_measures() {
        let mut bytes: Vec<u8> = vec![];
        let measures = Phases {
            current: PhaseMeasures {
                a: PhaseMeasurement{ value: 0, quality: 0},
                b: PhaseMeasurement{ value: 0, quality: 0},
                c: PhaseMeasurement{ value: 0, quality: 0},
                n: PhaseMeasurement{ value: 0, quality: 0},
            },
            voltage: PhaseMeasures {
                a: PhaseMeasurement{ value: 0, quality: 0},
                b: PhaseMeasurement{ value: 0, quality: 0},
                c: PhaseMeasurement{ value: 0, quality: 0},
                n: PhaseMeasurement{ value: 0, quality: 0},
            },
        };
        Asdu::measures_to_bytes(&mut bytes, &measures);
        let mut expected: Vec<u8> = vec![0x87, 0x40];
        expected.append(&mut vec![0; 64]);
        assert_eq!(bytes, expected);
    }
}