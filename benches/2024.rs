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
#[path = "../src/bin/2024/day6.rs"]
mod day6;
#[path = "../src/bin/2024/day7.rs"]
mod day7;
#[path = "../src/bin/2024/day8.rs"]
mod day8;
#[path = "../src/bin/2024/day9.rs"]
mod day9;
#[path = "../src/bin/2024/day10.rs"]
mod day10;
#[path = "../src/bin/2024/day11.rs"]
mod day11;
#[path = "../src/bin/2024/day12.rs"]
mod day12;
#[path = "../src/bin/2024/day13.rs"]
mod day13;
#[path = "../src/bin/2024/day14.rs"]
mod day14;
#[path = "../src/bin/2024/day15.rs"]
mod day15;
#[path = "../src/bin/2024/day16.rs"]
mod day16;
#[path = "../src/bin/2024/day17.rs"]
mod day17;
#[path = "../src/bin/2024/day18.rs"]
mod day18;
#[path = "../src/bin/2024/day19.rs"]
mod day19;
#[path = "../src/bin/2024/day20.rs"]
mod day20;
#[path = "../src/bin/2024/day21.rs"]
mod day21;
#[path = "../src/bin/2024/day22.rs"]
mod day22;
#[path = "../src/bin/2024/day23.rs"]
mod day23;
#[path = "../src/bin/2024/day24.rs"]
mod day24;
// NEXT MOD

day!(2024, 1, day1);
day!(2024, 2, day2);
day!(2024, 3, day3);
day!(2024, 4, day4);
day!(2024, 5, day5);
day!(2024, 6, day6);
day!(2024, 7, day7);
day!(2024, 8, day8);
day!(2024, 9, day9);
day!(2024, 10, day10);
day!(2024, 11, day11);
day!(2024, 12, day12);
day!(2024, 13, day13);
day!(2024, 14, day14);
day!(2024, 15, day15);
day!(2024, 16, day16);
day!(2024, 17, day17);
day!(2024, 18, day18);
day!(2024, 19, day19);
day!(2024, 20, day20);
day!(2024, 21, day21);
day!(2024, 22, day22);
day!(2024, 23, day23);
day!(2024, 24, day24);
// NEXT DAY

fn bench_2024(c: &mut criterion::Criterion) {
    c.bench_function("2024", |b| {
        b.iter(|| {
            day_combined!(2024, 1, day1);
            day_combined!(2024, 2, day2);
            day_combined!(2024, 3, day3);
            day_combined!(2024, 4, day4);
            day_combined!(2024, 5, day5);
            day_combined!(2024, 6, day6);
            day_combined!(2024, 7, day7);
            day_combined!(2024, 8, day8);
            day_combined!(2024, 9, day9);
            day_combined!(2024, 10, day10);
            day_combined!(2024, 11, day11);
            day_combined!(2024, 12, day12);
            day_combined!(2024, 13, day13);
            day_combined!(2024, 14, day14);
            day_combined!(2024, 15, day15);
            day_combined!(2024, 16, day16);
            day_combined!(2024, 17, day17);
            day_combined!(2024, 18, day18);
            day_combined!(2024, 19, day19);
            day_combined!(2024, 20, day20);
            day_combined!(2024, 21, day21);
            day_combined!(2024, 22, day22);
            day_combined!(2024, 23, day23);
            day_combined!(2024, 24, day24);
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
    bench_2024day6,
    bench_2024day7,
    bench_2024day8,
    bench_2024day9,
    bench_2024day10,
    bench_2024day11,
    bench_2024day12,
    bench_2024day13,
    bench_2024day14,
    bench_2024day15,
    bench_2024day16,
    bench_2024day17,
    bench_2024day18,
    bench_2024day19,
    bench_2024day20,
    bench_2024day21,
    bench_2024day22,
    bench_2024day23,
    bench_2024day24,
    // NEXT GROUP
);
