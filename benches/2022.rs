#[macro_use]
mod helpers;

#[path = "../src/bin/2022/day1.rs"]
mod day1;
#[path = "../src/bin/2022/day10.rs"]
mod day10;
#[path = "../src/bin/2022/day11.rs"]
mod day11;
#[path = "../src/bin/2022/day12.rs"]
mod day12;
#[path = "../src/bin/2022/day2.rs"]
mod day2;
#[path = "../src/bin/2022/day3.rs"]
mod day3;
#[path = "../src/bin/2022/day4.rs"]
mod day4;
#[path = "../src/bin/2022/day5.rs"]
mod day5;
#[path = "../src/bin/2022/day6.rs"]
mod day6;
#[path = "../src/bin/2022/day7.rs"]
mod day7;
#[path = "../src/bin/2022/day8.rs"]
mod day8;
#[path = "../src/bin/2022/day9.rs"]
mod day9;
#[path = "../src/bin/2022/day13.rs"]
mod day13;
#[path = "../src/bin/2022/day14.rs"]
mod day14;
#[path = "../src/bin/2022/day15.rs"]
mod day15;
#[path = "../src/bin/2022/day16.rs"]
mod day16;
#[path = "../src/bin/2022/day17.rs"]
mod day17;
#[path = "../src/bin/2022/day18.rs"]
mod day18;
#[path = "../src/bin/2022/day19.rs"]
mod day19;
// NEXT MOD

day!(2022, 1, day1);
day!(2022, 2, day2);
day!(2022, 3, day3);
day!(2022, 4, day4);
day!(2022, 5, day5);
day!(2022, 6, day6);
day!(2022, 7, day7);
day!(2022, 8, day8);
day!(2022, 9, day9);
day!(2022, 10, day10);
day!(2022, 11, day11);
day!(2022, 12, day12);
day!(2022, 13, day13);
day!(2022, 14, day14);
day!(2022, 15, day15);
day!(2022, 16, day16);
day!(2022, 17, day17);
day!(2022, 18, day18);
day!(2022, 19, day19);
// NEXT DAY

fn bench_2022(c: &mut criterion::Criterion) {
    c.bench_function("2022", |b| {
        b.iter(|| {
            day_combined!(2022, 1, day1);
            day_combined!(2022, 2, day2);
            day_combined!(2022, 3, day3);
            day_combined!(2022, 4, day4);
            day_combined!(2022, 5, day5);
            day_combined!(2022, 6, day6);
            day_combined!(2022, 7, day7);
            day_combined!(2022, 8, day8);
            day_combined!(2022, 9, day9);
            day_combined!(2022, 10, day10);
            day_combined!(2022, 11, day11);
            day_combined!(2022, 12, day12);
            day_combined!(2022, 13, day13);
            day_combined!(2022, 14, day14);
            day_combined!(2022, 15, day15);
            day_combined!(2022, 16, day16);
            day_combined!(2022, 17, day17);
            day_combined!(2022, 18, day18);
            day_combined!(2022, 19, day19);
            // NEXT DAY COMBINED
        })
    });
}
criterion::criterion_group!(benches_2022, bench_2022);

criterion::criterion_main!(
    benches_2022,
    bench_2022day1,
    bench_2022day2,
    bench_2022day3,
    bench_2022day4,
    bench_2022day5,
    bench_2022day6,
    bench_2022day7,
    bench_2022day8,
    bench_2022day9,
    bench_2022day10,
    bench_2022day11,
    bench_2022day12,
    bench_2022day13,
    bench_2022day14,
    bench_2022day15,
    bench_2022day16,
    bench_2022day17,
    bench_2022day18,
    bench_2022day19,
    // NEXT GROUP
);
