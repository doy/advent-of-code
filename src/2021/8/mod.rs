pub fn part1() -> anyhow::Result<i64> {
    let mut count = 0i64;
    for line in data_lines!()? {
        let mut parts = line.split(" | ");
        let output = parts.nth(1).unwrap();
        let line_count: i64 = output
            .split(' ')
            .filter(|s| [2, 3, 4, 7].contains(&s.len()))
            .count()
            .try_into()
            .unwrap();
        count += line_count;
    }
    Ok(count)
}

//  00
// 1  2
// 1  2
//  33
// 4  5
// 4  5
//  66
pub fn part2() -> anyhow::Result<i64> {
    let mut total = 0;
    for line in data_lines!()? {
        let mut parts = line.split(" | ");
        let numbers = parts.next().unwrap();
        let numbers: Vec<_> = numbers.split(' ').collect();

        let mut segments = ['x'; 7];

        // zero: 6
        let one = numbers.iter().find(|s| s.len() == 2).unwrap();
        let one: std::collections::HashSet<char> = one.chars().collect();
        // two: 5
        // three: 5
        let four = numbers.iter().find(|s| s.len() == 4).unwrap();
        let four: std::collections::HashSet<char> = four.chars().collect();
        // five: 5
        // six: 6
        let seven = numbers.iter().find(|s| s.len() == 3).unwrap();
        let seven: std::collections::HashSet<char> = seven.chars().collect();
        let eight = numbers.iter().find(|s| s.len() == 7).unwrap();
        let eight: std::collections::HashSet<char> = eight.chars().collect();
        // nine: 6

        let mut length_five: Vec<_> = numbers
            .iter()
            .filter_map(|s| {
                if s.len() == 5 {
                    Some(s.chars().collect::<std::collections::HashSet<_>>())
                } else {
                    None
                }
            })
            .collect();
        let mut length_six: Vec<_> = numbers
            .iter()
            .filter_map(|s| {
                if s.len() == 6 {
                    Some(s.chars().collect::<std::collections::HashSet<_>>())
                } else {
                    None
                }
            })
            .collect();

        let idx = length_five
            .iter()
            .position(|set| set.difference(&one).count() == 3)
            .unwrap();
        let three = length_five.swap_remove(idx);
        let idx = length_five
            .iter()
            .position(|set| set.difference(&four).count() == 2)
            .unwrap();
        let five = length_five.swap_remove(idx);
        let two = length_five.remove(0);

        segments[0] = *seven.difference(&one).next().unwrap();
        segments[6] = three
            .iter()
            .copied()
            .find(|c| {
                !one.contains(c) && !four.contains(c) && !seven.contains(c)
            })
            .unwrap();
        segments[3] = three
            .iter()
            .copied()
            .find(|c| {
                !one.contains(c) && *c != segments[0] && *c != segments[6]
            })
            .unwrap();
        segments[4] = two
            .iter()
            .copied()
            .find(|c| {
                !one.contains(c)
                    && *c != segments[0]
                    && *c != segments[3]
                    && *c != segments[6]
            })
            .unwrap();
        segments[2] = two
            .iter()
            .copied()
            .find(|c| {
                *c != segments[0]
                    && *c != segments[3]
                    && *c != segments[4]
                    && *c != segments[6]
            })
            .unwrap();
        segments[1] = four
            .iter()
            .copied()
            .find(|c| !one.contains(c) && *c != segments[3])
            .unwrap();
        segments[5] = eight
            .iter()
            .copied()
            .find(|c| {
                *c != segments[0]
                    && *c != segments[1]
                    && *c != segments[2]
                    && *c != segments[3]
                    && *c != segments[4]
                    && *c != segments[6]
            })
            .unwrap();

        let idx = length_six
            .iter()
            .position(|set| !set.contains(&segments[3]))
            .unwrap();
        let zero = length_six.swap_remove(idx);
        let idx = length_six
            .iter()
            .position(|set| !set.contains(&segments[2]))
            .unwrap();
        let six = length_six.swap_remove(idx);
        let idx = length_six
            .iter()
            .position(|set| !set.contains(&segments[4]))
            .unwrap();
        let nine = length_six.swap_remove(idx);

        let numbers =
            [zero, one, two, three, four, five, six, seven, eight, nine];

        let output = parts.next().unwrap();
        let value: Vec<_> = output
            .split(' ')
            .map(|s| s.chars().collect::<std::collections::HashSet<_>>())
            .map(|set| numbers.iter().position(|num| &set == num).unwrap())
            .collect();
        let value =
            value[0] * 1000 + value[1] * 100 + value[2] * 10 + value[3];
        total += value;
    }
    Ok(total.try_into()?)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 355);
    assert_eq!(part2().unwrap(), 983030);
}
