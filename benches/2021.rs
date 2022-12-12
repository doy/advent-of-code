#[macro_use]
mod helpers;

#[path = "../src/bin/2021/day1.rs"]
mod day1;
#[path = "../src/bin/2021/day10.rs"]
mod day10;
#[path = "../src/bin/2021/day11.rs"]
mod day11;
#[path = "../src/bin/2021/day12.rs"]
mod day12;
#[path = "../src/bin/2021/day13.rs"]
mod day13;
#[path = "../src/bin/2021/day14.rs"]
mod day14;
#[path = "../src/bin/2021/day15.rs"]
mod day15;
#[path = "../src/bin/2021/day16.rs"]
mod day16;
#[path = "../src/bin/2021/day17.rs"]
mod day17;
#[path = "../src/bin/2021/day18.rs"]
mod day18;
#[path = "../src/bin/2021/day19.rs"]
mod day19;
#[path = "../src/bin/2021/day2.rs"]
mod day2;
#[path = "../src/bin/2021/day20.rs"]
mod day20;
#[path = "../src/bin/2021/day21.rs"]
mod day21;
#[path = "../src/bin/2021/day22.rs"]
mod day22;
#[path = "../src/bin/2021/day23.rs"]
mod day23;
#[path = "../src/bin/2021/day24.rs"]
mod day24;
#[path = "../src/bin/2021/day25.rs"]
mod day25;
#[path = "../src/bin/2021/day3.rs"]
mod day3;
#[path = "../src/bin/2021/day4.rs"]
mod day4;
#[path = "../src/bin/2021/day5.rs"]
mod day5;
#[path = "../src/bin/2021/day6.rs"]
mod day6;
#[path = "../src/bin/2021/day7.rs"]
mod day7;
#[path = "../src/bin/2021/day8.rs"]
mod day8;
#[path = "../src/bin/2021/day9.rs"]
mod day9;
// NEXT MOD

day!(2021, 1, day1);
day!(2021, 2, day2);
day!(2021, 3, day3);
day!(2021, 4, day4);
day!(2021, 5, day5);
day!(2021, 6, day6);
day!(2021, 7, day7);
day!(2021, 8, day8);
day!(2021, 9, day9);
day!(2021, 10, day10);
day!(2021, 11, day11);
day!(2021, 12, day12);
day!(2021, 13, day13);
day!(2021, 14, day14);
day!(2021, 15, day15);
day!(2021, 16, day16);
day!(2021, 17, day17);
day!(2021, 18, day18);
day!(2021, 19, day19);
day!(2021, 20, day20);
day!(2021, 21, day21);
day!(2021, 22, day22);
day!(2021, 23, day23);
day!(2021, 24, day24);
day!(2021, 25, day25);
// NEXT DAY

criterion::criterion_main!(
    bench_2021day1,
    bench_2021day2,
    bench_2021day3,
    bench_2021day4,
    bench_2021day5,
    bench_2021day6,
    bench_2021day7,
    bench_2021day8,
    bench_2021day9,
    bench_2021day10,
    bench_2021day11,
    bench_2021day12,
    bench_2021day13,
    bench_2021day14,
    bench_2021day15,
    bench_2021day16,
    bench_2021day17,
    bench_2021day18,
    bench_2021day19,
    bench_2021day20,
    bench_2021day21,
    bench_2021day22,
    bench_2021day23,
    bench_2021day24,
    bench_2021day25,
    // NEXT GROUP
);
