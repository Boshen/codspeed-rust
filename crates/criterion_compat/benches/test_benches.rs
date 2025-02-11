use codspeed_criterion_compat::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench(c: &mut Criterion) {
    // Setup (construct data, allocate memory, etc)
    let input = 5u64;
    c.bench_with_input(BenchmarkId::new("with_input", input), &input, |b, i| {
        b.iter(|| {
            let mut x = 0;
            for _ in 0..*i {
                x += 2;
            }
            x
        })
    });
}

mod nested {
    use super::*;
    pub fn bench(c: &mut Criterion) {
        // Setup (construct data, allocate memory, etc)
        let input = 5u64;
        c.bench_with_input(BenchmarkId::new("with_input", input), &input, |b, i| {
            b.iter(|| {
                let mut x = 0;
                for _ in 0..*i {
                    x += 2;
                }
                x
            })
        });
    }
}

criterion_group!(benches, bench, nested::bench);
criterion_main!(benches);
