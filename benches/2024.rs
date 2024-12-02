#[macro_use]
mod helpers;

#[path = "../src/bin/2024/day1.rs"]
mod day1;
#[path = "../src/bin/2024/day2.rs"]
mod day2;
// NEXT MOD

day!(2024, 1, day1);
day!(2024, 2, day2);
// NEXT DAY

fn bench_2024(c: &mut criterion::Criterion) {
    c.bench_function("2024", |b| {
        b.iter(|| {
            day_combined!(2024, 1, day1);
            day_combined!(2024, 2, day2);
            // NEXT DAY COMBINED
        })
    });
}
criterion::criterion_group!(benches_2024, bench_2024);

criterion::criterion_main!(
    benches_2024,
    bench_2024day1,
    bench_2024day2,
    // NEXT GROUP
);
