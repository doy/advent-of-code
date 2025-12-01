#[macro_use]
mod helpers;

// NEXT MOD

// NEXT DAY

fn bench_2024(c: &mut criterion::Criterion) {
    c.bench_function("2024", |b| {
        b.iter(|| {
            // NEXT DAY COMBINED
        })
    });
}
criterion::criterion_group!(benches_2024, bench_2024);

criterion::criterion_main!(
    benches_2024,
    // NEXT GROUP
);
