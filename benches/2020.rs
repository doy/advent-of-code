#[macro_use]
mod helpers;

#[path = "../src/bin/2020/day1.rs"]
mod day1;
#[path = "../src/bin/2020/day2.rs"]
mod day2;
#[path = "../src/bin/2020/day3.rs"]
mod day3;
#[path = "../src/bin/2020/day4.rs"]
mod day4;
#[path = "../src/bin/2020/day5.rs"]
mod day5;
#[path = "../src/bin/2020/day6.rs"]
mod day6;
#[path = "../src/bin/2020/day7.rs"]
mod day7;
#[path = "../src/bin/2020/day8.rs"]
mod day8;
#[path = "../src/bin/2020/day9.rs"]
mod day9;
// NEXT MOD

day!(2020, 1, day1);
day!(2020, 2, day2);
day!(2020, 3, day3);
day!(2020, 4, day4);
day!(2020, 5, day5);
day!(2020, 6, day6);
day!(2020, 7, day7);
day!(2020, 8, day8);
day!(2020, 9, day9);
// NEXT DAY

criterion::criterion_main!(
    bench_2020day1,
    bench_2020day2,
    bench_2020day3,
    bench_2020day4,
    bench_2020day5,
    bench_2020day6,
    bench_2020day7,
    bench_2020day8,
    bench_2020day9,
    // NEXT GROUP
);
