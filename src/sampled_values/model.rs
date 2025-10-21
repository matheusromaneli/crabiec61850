pub enum PDUTags {
    PDU = 0x60,
    NoAsdu = 0x80,
    SeqAsdu = 0xa2,
    ASDU = 0x30,
}

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

#[derive(PartialEq, Debug)]
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
pub struct Asdu{
    pub sv_id: String,
    pub dataset: String,
    pub smp_count: u16,
    pub conf_rev: u32,
    pub refr_tm: Option<u64>,
    pub smp_sync: SampleSync,
    pub smp_rate: Option<u16>,
    pub measures: Vec<PhaseMeasurement>,
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