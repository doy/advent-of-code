macro_rules! day {
    ($year:expr, $day:expr, $mod:ident) => {
        paste::paste! {
            #[allow(unused_must_use)]
            fn [<bench_ $year $mod _part1>](c: &mut criterion::Criterion) {
                c.bench_function(
                    &format!("{} day {} part 1", $year, $day),
                    |b| {
                        b.iter(
                            || $mod::part1(
                                $mod::parse(advent_of_code::parse::data($year, $day).unwrap())
                                    .unwrap(),
                            )
                            .unwrap()
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
                        b.iter(
                            || $mod::part2(
                                $mod::parse(advent_of_code::parse::data($year, $day).unwrap())
                                    .unwrap(),
                            )
                            .unwrap()
                        )
                    }
                );
            }
        }

        paste::paste! {
            criterion::criterion_group!(
                [<bench_ $year $mod>],
                [<bench_ $year $mod _part1>],
                [<bench_ $year $mod _part2>],
            );
        }
    };
}

macro_rules! day_combined {
    ($year:expr, $day:expr, $mod:ident) => {{
        $mod::part1(
            $mod::parse(advent_of_code::parse::data($year, $day).unwrap())
                .unwrap(),
        )
        .unwrap();
        $mod::part2(
            $mod::parse(advent_of_code::parse::data($year, $day).unwrap())
                .unwrap(),
        )
        .unwrap();
    }};
}
