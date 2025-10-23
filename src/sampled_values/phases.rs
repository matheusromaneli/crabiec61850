use byteorder::{BigEndian, ByteOrder};

use crate::sampled_values::model::{PhaseMeasurement, PhaseMeasures, Phases};

impl PhaseMeasurement{
    pub fn from_bytes(bytes: &[u8]) -> PhaseMeasurement {
        if bytes.len() != 8 {
            panic!("PhaseMeasurement must be 8 bytes long");
        }
        PhaseMeasurement {
            value: BigEndian::read_i32(&bytes[0..4]),
            quality: BigEndian::read_i32(&bytes[4..8]),
        }
    }
}

impl PhaseMeasures {
    pub fn from_bytes(bytes: &[u8]) -> PhaseMeasures {
        if bytes.len() != 32 {
            panic!("PhaseMeasures must be 32 bytes long");
        }
        PhaseMeasures {
            a: PhaseMeasurement::from_bytes(&bytes[0..8]),
            b: PhaseMeasurement::from_bytes(&bytes[8..16]),
            c: PhaseMeasurement::from_bytes(&bytes[16..24]),
            n: PhaseMeasurement::from_bytes(&bytes[24..32]),
        }
    }
}

impl Phases {
    pub fn from_bytes(bytes: &[u8]) -> Phases {
        if bytes.len() != 64 {
            panic!("Phases must be 64 bytes long");
        }
        Phases {
            current: PhaseMeasures::from_bytes(&bytes[0..32]),
            voltage: PhaseMeasures::from_bytes(&bytes[32..64]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_measurement_from_bytes() {
        let bytes: &[u8] = &[
            0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00,
        ];
        let phase_measurement = PhaseMeasurement::from_bytes(bytes);
        assert_eq!(phase_measurement.value, -3);
        assert_eq!(phase_measurement.quality, 0);
    }

    #[test]
    fn phase_measures_from_bytes() {
        let bytes: &[u8] = &[
            0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x20, 0x00,
        ];

        let phase_measures = PhaseMeasures::from_bytes(bytes);
        assert_eq!(phase_measures.a.value, -3);
        assert_eq!(phase_measures.a.quality, 0);
        assert_eq!(phase_measures.b.value, 3);
        assert_eq!(phase_measures.b.quality, 0);
        assert_eq!(phase_measures.c.value, 3);
        assert_eq!(phase_measures.c.quality, 0);
        assert_eq!(phase_measures.n.value, 3);
        assert_eq!(phase_measures.n.quality, 0x00002000);
    }

    #[test]
    fn phases_from_bytes() {
        let bytes: &[u8] = &[
            0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x20, 0x00,
            0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00,
            0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00,
            0xff, 0xff, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x00,
            0xff, 0xff, 0xff, 0xf6, 0x00, 0x00, 0x20, 0x00
        ];

        let phases = Phases::from_bytes(bytes);
        assert_eq!(phases.current.a.value, -3);
        assert_eq!(phases.current.a.quality, 0);
        assert_eq!(phases.current.b.value, 3);
        assert_eq!(phases.current.b.quality, 0);
        assert_eq!(phases.current.c.value, 3);
        assert_eq!(phases.current.c.quality, 0);
        assert_eq!(phases.current.n.value, 3);
        assert_eq!(phases.current.n.quality, 0x00002000);
        assert_eq!(phases.voltage.a.value, -3);
        assert_eq!(phases.voltage.a.quality, 0);
        assert_eq!(phases.voltage.b.value, -3);
        assert_eq!(phases.voltage.b.quality, 0);
        assert_eq!(phases.voltage.c.value, -4);
        assert_eq!(phases.voltage.c.quality, 0);
        assert_eq!(phases.voltage.n.value, -10);
        assert_eq!(phases.voltage.n.quality, 0x00002000);
        
    }
}