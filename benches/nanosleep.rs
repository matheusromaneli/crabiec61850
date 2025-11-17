
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use libc::nanosleep;


fn nanosleep_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("nanosleep");

    let mut rmtp = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    for i in [100_000, 200_000, 208_333]{
        rmtp.tv_nsec = i;
        group.bench_function(BenchmarkId::new("nanosleep", i), |b| {
            b.iter(|| {
                unsafe{ 
                    std::hint::black_box(nanosleep(&rmtp, core::ptr::null_mut()));
                };
            });
        });
    }

    group.finish();
}

criterion_group!(benches, nanosleep_benchmark);
criterion_main!(benches);