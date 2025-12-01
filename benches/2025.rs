#[macro_use]
mod helpers;

#[path = "../src/bin/2025/day1.rs"]
mod day1;
// NEXT MOD

day!(2025, 1, day1);
// NEXT DAY

fn bench_2024(c: &mut criterion::Criterion) {
    c.bench_function("2024", |b| {
        b.iter(|| {
            day_combined!(2025, 1, day1);
            // NEXT DAY COMBINED
        })
    });
}
criterion::criterion_group!(benches_2024, bench_2024);

criterion::criterion_main!(
    benches_2024,
    bench_2025day1,
    // NEXT GROUP
);
