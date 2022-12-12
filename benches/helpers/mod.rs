macro_rules! day {
    ($year:expr, $day:expr, $mod:ident) => {
        paste::paste! {
            #[allow(unused_must_use)]
            fn [<bench_ $year $mod _parse>](c: &mut criterion::Criterion) {
                c.bench_function(
                    &format!("{} day {} parse", $year, $day),
                    |b| {
                        b.iter_batched(
                            || advent_of_code::parse::data($year, $day).unwrap(),
                            |data| $mod::parse(criterion::black_box(data)),
                            criterion::BatchSize::PerIteration,
                        )
                    }
                );
            }
        }

        paste::paste! {
            #[allow(unused_must_use)]
            fn [<bench_ $year $mod _part1>](c: &mut criterion::Criterion) {
                c.bench_function(
                    &format!("{} day {} part 1", $year, $day),
                    |b| {
                        b.iter_batched(
                            || {
                                $mod::parse(
                                    advent_of_code::parse::data($year, $day)
                                        .unwrap(),
                                )
                                .unwrap()
                            },
                            |data| $mod::part1(criterion::black_box(data)),
                            criterion::BatchSize::PerIteration,
                        )
                    }
                );
            }
        }

        paste::paste! {
            #[allow(unused_must_use)]
            fn [<bench_ $year $mod _part2>](c: &mut criterion::Criterion) {
                c.bench_function(
                    &format!("{} day {} part 2", $year, $day),
                    |b| {
                        b.iter_batched(
                            || {
                                $mod::parse(
                                    advent_of_code::parse::data($year, $day)
                                        .unwrap(),
                                )
                                .unwrap()
                            },
                            |data| $mod::part2(criterion::black_box(data)),
                            criterion::BatchSize::PerIteration,
                        )
                    }
                );
            }
        }

        paste::paste! {
            criterion::criterion_group!(
                [<bench_ $year $mod>],
                [<bench_ $year $mod _parse>],
                [<bench_ $year $mod _part1>],
                [<bench_ $year $mod _part2>],
            );
        }
    };
}
