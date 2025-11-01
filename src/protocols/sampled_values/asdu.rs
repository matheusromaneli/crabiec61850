use byteorder::{BigEndian, ByteOrder};
use crate::{protocols::sampled_values::model::{ASDUTags, Asdu, Phases, SampleSync}, standards::asn1::Triplet};

impl Asdu {

    pub fn from_bytes(bytes: &[u8]) -> Asdu {
        let mut triplet = Triplet::from_bytes(bytes);
        let mut start: usize = triplet.len();
        let sv_id = String::from_utf8(triplet.value.to_vec()).unwrap();

        triplet = Triplet::from_bytes(&bytes[start..]);
        start += triplet.len();
        let mut dataset: Option<String> = None;
        if triplet.tag == 0x81 {
            dataset = Some(String::from_utf8(triplet.value.to_vec()).unwrap());
            
            triplet = Triplet::from_bytes(&bytes[start..]);
            start += triplet.len();
        }
        let smp_count: u16 = BigEndian::read_u16(&triplet.value);

        triplet = Triplet::from_bytes(&bytes[start..]);
        start += triplet.len();
        let conf_rev: u32 = BigEndian::read_u32(&triplet.value);

        triplet = Triplet::from_bytes(&bytes[start..]);
        start += triplet.len();
        let mut refr_tm: Option<u64> = None;
        if triplet.tag == 0x84 {
            refr_tm = Some(BigEndian::read_u64(&triplet.value));

            triplet = Triplet::from_bytes(&bytes[start..]);
            start += triplet.len();
        }
        let smp_sync: SampleSync = match triplet.value[0] {
            0 => SampleSync::Internal,
            1 => SampleSync::Local,
            2 => SampleSync::Global,
            _ => panic!("Invalid SampleSync value: {}", &triplet.value[0]),
        };

        triplet = Triplet::from_bytes(&bytes[start..]);
        start += triplet.len();
        let mut smp_rate: Option<u16> = None;
        if triplet.tag == 0x86 {
            smp_rate = Some(BigEndian::read_u16(&triplet.value));

            triplet = Triplet::from_bytes(&bytes[start..]);
            start += triplet.len();
        }
        let measures = Phases::from_bytes(&triplet.value);

    let mut smp_mode: Option<u16> = None;
        if start < bytes.len(){
            triplet = Triplet::from_bytes(&bytes[start..]);
            if triplet.tag == 0x88 {
                smp_mode = Some(BigEndian::read_u16(&triplet.value));
            }
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

    fn sv_id_to_bytes(sv_id: &str) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::SvId as u8);
        let sv_id_bytes = sv_id.as_bytes();
        bytes.push(sv_id_bytes.len() as u8);
        bytes.append(&mut sv_id_bytes.to_vec());
        bytes
    }

    fn dataset_to_bytes(dataset: &str) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::Dataset as u8);
        let dataset_bytes = dataset.as_bytes();
        bytes.push(dataset_bytes.len() as u8);
        bytes.append(&mut dataset_bytes.to_vec());
        bytes
    }

    fn smp_count_to_bytes(smp_count: u16) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::SmpCount as u8);
        bytes.push(2);
        bytes.append(&mut vec![0; 2]);
        BigEndian::write_u16(&mut bytes[2..], smp_count);
        bytes
    }

    fn conf_rev_to_bytes(conf_rev: u32) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::ConfRev as u8);
        bytes.push(4);
        bytes.append(&mut vec![0; 4]);
        BigEndian::write_u32(&mut bytes[2..], conf_rev);
        bytes
    }

    fn refr_tm_to_bytes(refr_tm: u64) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::RefrTm as u8);
        bytes.push(8);
        bytes.append(&mut vec![0; 8]);
        BigEndian::write_u64(&mut bytes[2..], refr_tm);
        bytes
    }

    fn smp_sync_to_bytes(smp_sync: u8) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::SmpSync as u8);
        bytes.push(1);
        bytes.push(smp_sync);
        bytes
    }

    fn smp_rate_to_bytes(smp_rate: u16) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::SmpRate as u8);
        bytes.push(2);
        BigEndian::write_u16(&mut bytes[2..], smp_rate);
        bytes
    }

    fn measures_to_bytes(measures: &Phases) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::Measures as u8);
        bytes.push(64);
        bytes.append(&mut measures.to_bytes());
        bytes
    }

    fn smp_mode_to_bytes(smp_mode: u16) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![];
        bytes.push(ASDUTags::SmpMode as u8);
        bytes.push(2);
        bytes.append(&mut vec![0; 2]);
        BigEndian::write_u16(&mut bytes[2..], smp_mode);
        bytes
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.append(&mut Self::sv_id_to_bytes(&self.sv_id));

        if let Some(dataset) = &self.dataset {
            bytes.append(&mut Self::dataset_to_bytes(dataset));
        }

        bytes.append(&mut Self::smp_count_to_bytes(self.smp_count));

        bytes.append(&mut Self::conf_rev_to_bytes(self.conf_rev));

        if let Some(refr_tm) = &self.refr_tm {
            bytes.append(&mut Self::refr_tm_to_bytes(*refr_tm));
        }

        bytes.append(&mut Self::smp_sync_to_bytes(self.smp_sync as u8));

        if let Some(smp_rate) = &self.smp_rate {
            bytes.append(&mut Self::smp_rate_to_bytes(*smp_rate));
        }

        bytes.append(&mut Self::measures_to_bytes(&self.measures));

        if let Some(smp_mode) = &self.smp_mode {
            bytes.append(&mut Self::smp_mode_to_bytes(*smp_mode));
        }

        bytes
    }

    pub fn next(&mut self) {
        let vol: f64 = 30.0;
        let amp: f64 = 0.0;
        let phase_angle: f64 = 0.0;

        self.smp_count = (self.smp_count + 1) % 4800;

        let sample_point: f64 = (self.smp_count % 80) as f64;

        let angle_a: f64 = sample_point * (2 as f64 * std::f64::consts::PI / 80 as f64);
        let angle_b: f64 = sample_point * (2 as f64 * std::f64::consts::PI / 80 as f64);
        let angle_c: f64 = sample_point * (2 as f64 * std::f64::consts::PI / 80 as f64);

        self.measures.current.a.value = (amp * (angle_a - phase_angle).sin() * 1000 as f64) as i32;
        self.measures.current.b.value = (amp * (angle_b - phase_angle).sin() * 1000 as f64) as i32;
        self.measures.current.c.value = (amp * (angle_c - phase_angle).sin() * 1000 as f64) as i32;
        self.measures.current.n.value = self.measures.current.a.value + self.measures.current.b.value + self.measures.current.c.value;

        self.measures.voltage.a.value = (vol * angle_a.sin() * 100 as f64) as i32;
        self.measures.voltage.b.value = (vol * angle_b.sin() * 100 as f64) as i32;
        self.measures.voltage.c.value = (vol * angle_c.sin() * 100 as f64) as i32;
        self.measures.voltage.n.value = self.measures.voltage.a.value + self.measures.voltage.b.value + self.measures.voltage.c.value;
    }
}

#[cfg(test)]
mod tests {
    use crate::protocols::sampled_values::model::{PhaseMeasurement, PhaseMeasures, Phases};

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
        assert_eq!(Asdu::sv_id_to_bytes("4000"), vec![0x80, 0x04, 0x34, 0x30, 0x30, 0x30]);
    }

    #[test]
    fn add_smp_count() {
        assert_eq!(Asdu::smp_count_to_bytes(0), vec![0x82, 0x02, 0x00, 0x00]);
    }
    
    #[test]
    fn add_conf_rev() {
        assert_eq!(Asdu::conf_rev_to_bytes(1), vec![0x83, 0x04, 0x00, 0x00, 0x00, 0x01]);
    }

    #[test]
    fn add_smp_sync() {
        assert_eq!(Asdu::smp_sync_to_bytes(SampleSync::Local as u8), vec![0x85, 0x01, 0x01]);
    }

    #[test]
    fn add_measures() {
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
        
        let mut expected: Vec<u8> = vec![0x87, 0x40];
        expected.append(&mut vec![0; 64]);
        assert_eq!(Asdu::measures_to_bytes(&measures), expected);
    }

    #[test]
    fn asdu_to_bytes(){
        let bytes: &[u8] = &[
            0x80, 0x04, 0x34, 0x30, 0x30, 0x30, 0x82, 0x02, 0x00, 0x00, 0x83, 0x04, 0x00, 0x00,
            0x00, 0x01, 0x85, 0x01, 0x01, 0x87, 0x40, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x00, 0x20, 0x00, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0xff,
            0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x00, 0xff,
            0xff, 0xff, 0xf6, 0x00, 0x00, 0x20, 0x00,
        ];

        let object = Asdu::from_bytes(bytes);
        assert_eq!(object.to_bytes(), bytes);
    }

    #[test]
    fn next_asdu(){
        let mut current_asdu = Asdu{
            sv_id: "4000".to_string(),
            dataset: None,
            smp_count: 0,
            conf_rev: 1,
            refr_tm: None,
            smp_sync: SampleSync::Local,
            smp_rate: None,
            measures: Phases {
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
            },
            smp_mode: None,
        };

        let expected_asdu_next = Asdu {
            sv_id: "4000".to_string(),
            dataset: None,
            smp_count: 1,
            conf_rev: 1,
            refr_tm: None,
            smp_sync: SampleSync::Local,
            smp_rate: None,
            measures: Phases {
                current: PhaseMeasures {
                    a: PhaseMeasurement{ value: 0, quality: 0},
                    b: PhaseMeasurement{ value: 0, quality: 0},
                    c: PhaseMeasurement{ value: 0, quality: 0},
                    n: PhaseMeasurement{ value: 0, quality: 0},
                },
                voltage: PhaseMeasures {
                    a: PhaseMeasurement{ value: 235, quality: 0},
                    b: PhaseMeasurement{ value: 235, quality: 0},
                    c: PhaseMeasurement{ value: 235, quality: 0},
                    n: PhaseMeasurement{ value: 705, quality: 0},
                },
            },
            smp_mode: None,
        };
        current_asdu.next();
        assert_eq!(current_asdu, expected_asdu_next);
    }

}