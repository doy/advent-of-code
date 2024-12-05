#[macro_use]
mod helpers;

#[path = "../src/bin/2024/day1.rs"]
mod day1;
#[path = "../src/bin/2024/day2.rs"]
mod day2;
#[path = "../src/bin/2024/day3.rs"]
mod day3;
#[path = "../src/bin/2024/day4.rs"]
mod day4;
#[path = "../src/bin/2024/day5.rs"]
mod day5;
// NEXT MOD

day!(2024, 1, day1);
day!(2024, 2, day2);
day!(2024, 3, day3);
day!(2024, 4, day4);
day!(2024, 5, day5);
// NEXT DAY

fn bench_2024(c: &mut criterion::Criterion) {
    c.bench_function("2024", |b| {
        b.iter(|| {
            day_combined!(2024, 1, day1);
            day_combined!(2024, 2, day2);
            day_combined!(2024, 3, day3);
            day_combined!(2024, 4, day4);
            day_combined!(2024, 5, day5);
            // NEXT DAY COMBINED
        })
    });
}
criterion::criterion_group!(benches_2024, bench_2024);

criterion::criterion_main!(
    benches_2024,
    bench_2024day1,
    bench_2024day2,
    bench_2024day3,
    bench_2024day4,
    bench_2024day5,
    // NEXT GROUP
);
