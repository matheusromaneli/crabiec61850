use std::time::Instant;

//use libc::nanosleep;

use crate::{
    network::{packet::Packet, socket::RawSocket},
    protocols::{ethernet::model::Ethernet, sampled_values::model::SampledValue},
};

pub fn main() {
    let socket = RawSocket::new("lo".to_string(), 0x88ba_u16);

    let sv_bytes: &[u8] = &[
        0x40, 0x02, 0x00, 0x66, 0x00, 0x00, 0x00, 0x00, // Header
        0x60, 0x5c, // PDU
        0x80, 0x01, 0x01, // number of ASDU
        0xa2, 0x57, // sequence of ASDU
        0x30, 0x55, 0x80, 0x04, 0x34, 0x30, 0x30, 0x30, 0x82, 0x02, 0x00, 0x00, 0x83, 0x04, 0x00,
        0x00, 0x00, 0x01, 0x85, 0x01, 0x01, 0x87, 0x40, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x20, 0x00, 0xff, 0xff, 0xff, 0xfd, 0x00,
        0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xfc,
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xf6, 0x00, 0x00, 0x20, 0x00, // ASDU
    ];
    let mut config = Packet {
        ether_type: [0x88, 0xba],
        ethernet: Ethernet {
            dst_mac: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            src_mac: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ether_type: [0x88, 0xba],
        },
        sampled_value: SampledValue::from_bytes(sv_bytes),
    };
    let time_between_packets = 208_333;
    let time_next_perf = 16;
    let time_to_bytes_perf = 909;
    let time_send_perf = 1_500;
    let precision_diff_time = 55_750;
    let _tm_spec = libc::timespec {
        tv_sec: 0,
        tv_nsec: time_between_packets
            - time_next_perf
            - time_to_bytes_perf
            - time_send_perf
            - precision_diff_time,
    };
    let mut last = Instant::now();
    let mut now = Instant::now();
    let mut bytes = config.to_bytes();
    let mut diff: i64;
    let _time_sleep = time_between_packets - 2;
    let mut compensation: i64 = 0;

    loop {
        // sleep
        // unsafe { nanosleep(&_tm_spec, core::ptr::null_mut()) }; // uncomment import nanosleep
        // socket.send(&config.to_bytes());
        // config.sampled_value.next();
        // end sleep

        // busy-wait
        diff = now.duration_since(last).as_nanos() as i64;
        if diff >= _time_sleep-compensation {
            socket.send(&bytes);
            compensation = diff - _time_sleep;
            if compensation < 0 {
                compensation = 0;
            }
            last = now;
            config.sampled_value.next();
            bytes = config.to_bytes();
        }
        now = Instant::now();
        // end busy-wait
    }
}
