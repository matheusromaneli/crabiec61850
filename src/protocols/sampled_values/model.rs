#[repr(u8)]
pub enum PDUTags {
    PDU = 0x60,
    NoAsdu = 0x80,
    SeqAsdu = 0xa2,
    ASDU = 0x30,
}

#[repr(u8)]
pub enum ASDUTags {
    SvId = 0x80,
    Dataset = 0x81,
    SmpCount = 0x82,
    ConfRev = 0x83,
    RefrTm = 0x84,
    SmpSync = 0x85,
    SmpRate = 0x86,
    Measures = 0x87,
    SmpMode = 0x88,
}

#[repr(u8)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SampleSync{
    /*
    Sampled values synchronization source.
    See IEC 61850-7-2.
    */
    Internal = 0,
    Local = 1,
    Global = 2,
}

#[derive(PartialEq, Debug)]
pub struct PhaseMeasurement {
    pub value: i32,
    pub quality: i32,
}

#[derive(PartialEq, Debug)]
pub struct PhaseMeasures {
    pub a: PhaseMeasurement,
    pub b: PhaseMeasurement,
    pub c: PhaseMeasurement,
    pub n: PhaseMeasurement,
}

#[derive(PartialEq, Debug)]
pub struct Phases {
    pub current: PhaseMeasures,
    pub voltage: PhaseMeasures,
}

#[derive(PartialEq, Debug)]
pub struct Asdu{
    pub sv_id: String,
    pub dataset: Option<String>,
    pub smp_count: u16,
    pub conf_rev: u32,
    pub refr_tm: Option<u64>,
    pub smp_sync: SampleSync,
    pub smp_rate: Option<u16>,
    pub measures: Phases,
    pub smp_mode: Option<u16>,
}


#[derive(PartialEq, Debug)]
pub struct SampledValue{
    pub app_id: u16,
    pub length: u16,
    pub simulation: bool,
    pub reserved1: [u8;2],
    pub reserved2: [u8;2],
    pub number_of_asdu: u16,
    pub asdu: Vec<Asdu>
}