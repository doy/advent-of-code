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
#[path = "../src/bin/2023/day5.rs"]
mod day5;
#[path = "../src/bin/2023/day6.rs"]
mod day6;
#[path = "../src/bin/2023/day7.rs"]
mod day7;
#[path = "../src/bin/2023/day8.rs"]
mod day8;
#[path = "../src/bin/2023/day9.rs"]
mod day9;
#[path = "../src/bin/2023/day10.rs"]
mod day10;
#[path = "../src/bin/2023/day11.rs"]
mod day11;
#[path = "../src/bin/2023/day12.rs"]
mod day12;
#[path = "../src/bin/2023/day13.rs"]
mod day13;
#[path = "../src/bin/2023/day14.rs"]
mod day14;
#[path = "../src/bin/2023/day15.rs"]
mod day15;
#[path = "../src/bin/2023/day16.rs"]
mod day16;
#[path = "../src/bin/2023/day17.rs"]
mod day17;
#[path = "../src/bin/2023/day18.rs"]
mod day18;
// NEXT MOD

day!(2023, 1, day1);
day!(2023, 2, day2);
day!(2023, 3, day3);
day!(2023, 4, day4);
day!(2023, 5, day5);
day!(2023, 6, day6);
day!(2023, 7, day7);
day!(2023, 8, day8);
day!(2023, 9, day9);
day!(2023, 10, day10);
day!(2023, 11, day11);
day!(2023, 12, day12);
day!(2023, 13, day13);
day!(2023, 14, day14);
day!(2023, 15, day15);
day!(2023, 16, day16);
day!(2023, 17, day17);
day!(2023, 18, day18);
// NEXT DAY

fn bench_2023(c: &mut criterion::Criterion) {
    c.bench_function("2023", |b| {
        b.iter(|| {
            day_combined!(2023, 1, day1);
            day_combined!(2023, 2, day2);
            day_combined!(2023, 3, day3);
            day_combined!(2023, 4, day4);
            day_combined!(2023, 5, day5);
            day_combined!(2023, 6, day6);
            day_combined!(2023, 7, day7);
            day_combined!(2023, 8, day8);
            day_combined!(2023, 9, day9);
            day_combined!(2023, 10, day10);
            day_combined!(2023, 11, day11);
            day_combined!(2023, 12, day12);
            day_combined!(2023, 13, day13);
            day_combined!(2023, 14, day14);
            day_combined!(2023, 15, day15);
            day_combined!(2023, 16, day16);
            day_combined!(2023, 17, day17);
            day_combined!(2023, 18, day18);
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
    bench_2023day5,
    bench_2023day6,
    bench_2023day7,
    bench_2023day8,
    bench_2023day9,
    bench_2023day10,
    bench_2023day11,
    bench_2023day12,
    bench_2023day13,
    bench_2023day14,
    bench_2023day15,
    bench_2023day16,
    bench_2023day17,
    bench_2023day18,
    // NEXT GROUP
);
