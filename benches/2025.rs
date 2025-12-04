#[macro_use]
mod helpers;

#[path = "../src/bin/2025/day1.rs"]
mod day1;
#[path = "../src/bin/2025/day2.rs"]
mod day2;
#[path = "../src/bin/2025/day3.rs"]
mod day3;
#[path = "../src/bin/2025/day4.rs"]
mod day4;
// NEXT MOD

day!(2025, 1, day1);
day!(2025, 2, day2);
day!(2025, 3, day3);
day!(2025, 4, day4);
// NEXT DAY

fn bench_2024(c: &mut criterion::Criterion) {
    c.bench_function("2024", |b| {
        b.iter(|| {
            day_combined!(2025, 1, day1);
            day_combined!(2025, 2, day2);
            day_combined!(2025, 3, day3);
            day_combined!(2025, 4, day4);
            // NEXT DAY COMBINED
        })
    });
}
criterion::criterion_group!(benches_2024, bench_2024);

criterion::criterion_main!(
    benches_2024,
    bench_2025day1,
    bench_2025day2,
    bench_2025day3,
    bench_2025day4,
    // NEXT GROUP
);
