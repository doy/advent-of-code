#[macro_use]
mod helpers;

#[path = "../src/bin/2023/day1.rs"]
mod day1;
#[path = "../src/bin/2023/day2.rs"]
mod day2;
#[path = "../src/bin/2023/day3.rs"]
mod day3;
#[path = "../src/bin/2023/day4.rs"]
mod day4;
// NEXT MOD

day!(2023, 1, day1);
day!(2023, 2, day2);
day!(2023, 3, day3);
day!(2023, 4, day4);
// NEXT DAY

fn bench_2023(c: &mut criterion::Criterion) {
    c.bench_function("2023", |b| {
        b.iter(|| {
            day_combined!(2023, 1, day1);
            day_combined!(2023, 2, day2);
            day_combined!(2023, 3, day3);
            day_combined!(2023, 4, day4);
            // NEXT DAY COMBINED
        })
    });
}
criterion::criterion_group!(benches_2023, bench_2023);

criterion::criterion_main!(
    benches_2023,
    bench_2023day1,
    bench_2023day2,
    bench_2023day3,
    bench_2023day4,
    // NEXT GROUP
);
