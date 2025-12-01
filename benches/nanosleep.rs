use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use libc::nanosleep;

fn nanosleep_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("nanosleep");

    let mut rmtp = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let time_between_packets = 208_333;
    let time_next_perf = 16;
    let time_to_bytes_perf = 909;
    let time_send_perf = 1_500;
    let precision_diff_time = 55_750;
    let sv_time_sleep_estimation: i64 = time_between_packets
        - time_next_perf
        - time_to_bytes_perf
        - time_send_perf
        - precision_diff_time;

    for i in [100_000, 200_000, 208_333, sv_time_sleep_estimation] {
        rmtp.tv_nsec = i;
        group.bench_function(BenchmarkId::new("nanosleep", i), |b| {
            b.iter(|| {
                unsafe {
                    std::hint::black_box(nanosleep(&rmtp, core::ptr::null_mut()));
                };
            });
        });
    }

    group.finish();
}

criterion_group!(benches, nanosleep_benchmark);
criterion_main!(benches);
