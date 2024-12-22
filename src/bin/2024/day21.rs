use advent_of_code::prelude::*;

const NUMPAD: [[&str; 11]; 11] = [
    [
        "", "^<", "^", ">^", "^^<", "^^", ">^^", "^^^<", "^^^", ">^^^", ">",
    ],
    [
        ">v", "", ">", ">>", "^", ">^", ">>^", "^^", ">^^", ">>^^", ">>v",
    ],
    ["v", "<", "", ">", "<^", "^", ">^", "<^^", "^^", ">^^", "v>"],
    [
        "<v", "<<", "<", "", "<<^", "<^", "^", "<<^^", "<^^", "^^", "v",
    ],
    [
        ">vv", "v", "v>", "v>>", "", ">", ">>", "^", ">^", ">>^", ">>vv",
    ],
    ["vv", "<v", "v", "v>", "<", "", ">", "<^", "^", ">^", "vv>"],
    [
        "<vv", "<<v", "<v", "v", "<<", "<", "", "<<^", "<^", "^", "vv",
    ],
    [
        ">vvv", "vv", "vv>", "vv>>", "v", "v>", "v>>", "", ">", ">>", ">>vvv",
    ],
    [
        "vvv", "<vv", "vv", "vv>", "<v", "v", "v>", "<", "", ">", "vvv>",
    ],
    [
        "<vvv", "<<vv", "<vv", "vv", "<<v", "<v", "v", "<<", "<", "", "vvv",
    ],
    [
        "<", "^<<", "<^", "^", "^^<<", "<^^", "^^", "^^^<<", "<^^^", "^^^",
        "",
    ],
];
const NUMPAD_0: usize = 0;
const NUMPAD_1: usize = 1;
const NUMPAD_2: usize = 2;
const NUMPAD_3: usize = 3;
const NUMPAD_4: usize = 4;
const NUMPAD_5: usize = 5;
const NUMPAD_6: usize = 6;
const NUMPAD_7: usize = 7;
const NUMPAD_8: usize = 8;
const NUMPAD_9: usize = 9;
const NUMPAD_A: usize = 10;

const ARROWS: [[&str; 5]; 5] = [
    ["", ">^", ">", ">>", ">>^"],
    ["v<", "", "v", "v>", ">"],
    ["<", "^", "", ">", ">^"],
    ["<<", "<^", "<", "", "^"],
    ["v<<", "<", "v<", "v", ""],
];
const ARROW_LEFT: usize = 0;
const ARROW_UP: usize = 1;
const ARROW_DOWN: usize = 2;
const ARROW_RIGHT: usize = 3;
const ARROW_A: usize = 4;

fn numpad_char(c: char) -> usize {
    match c {
        '0' => NUMPAD_0,
        '1' => NUMPAD_1,
        '2' => NUMPAD_2,
        '3' => NUMPAD_3,
        '4' => NUMPAD_4,
        '5' => NUMPAD_5,
        '6' => NUMPAD_6,
        '7' => NUMPAD_7,
        '8' => NUMPAD_8,
        '9' => NUMPAD_9,
        'A' => NUMPAD_A,
        _ => unreachable!(),
    }
}

fn arrow_char(c: char) -> usize {
    match c {
        '<' => ARROW_LEFT,
        '^' => ARROW_UP,
        'v' => ARROW_DOWN,
        '>' => ARROW_RIGHT,
        'A' => ARROW_A,
        _ => unreachable!(),
    }
}

fn complexity(s: i64, orig: &str) -> i64 {
    s * orig[..orig.len() - 1].parse::<i64>().unwrap()
}

fn calculate_number_paths_for(
    pairs: &[(usize, usize)],
) -> Vec<HashMap<(usize, usize), String>> {
    fn calculate_number_paths_for_rec(
        base: &[[&str; 11]; 11],
        pairs: &[(usize, usize)],
        idx: usize,
    ) -> Vec<HashMap<(usize, usize), String>> {
        if idx >= pairs.len() {
            return vec![HashMap::new()];
        }
        let next = calculate_number_paths_for_rec(base, pairs, idx + 1);
        let (from, to) = pairs[idx];
        let mut permutations = permutations(base[from][to].as_bytes());
        permutations.sort_unstable();
        permutations.dedup();
        let mut ret = vec![];
        for permutation in permutations
            .into_iter()
            .map(|s| String::from_utf8(s).unwrap())
        {
            if !illegal_numpad_move(from, &permutation) {
                ret.extend(next.iter().cloned().map(|mut map| {
                    map.insert((from, to), permutation.clone());
                    map
                }))
            }
        }
        ret
    }

    calculate_number_paths_for_rec(&NUMPAD, pairs, 0)
}

fn calculate_all_arrow_paths() -> Vec<[[String; 5]; 5]> {
    fn calculate_all_arrow_paths_rec(
        base: &[[String; 5]; 5],
        from: usize,
        to: usize,
    ) -> Vec<[[String; 5]; 5]> {
        if from == 5 {
            return vec![base.clone()];
        }
        let (next_from, next_to) = if to == 4 {
            (from + 1, 0)
        } else {
            (from, to + 1)
        };
        let mut ret = vec![];
        let mut next = base.clone();
        let mut permutations = permutations(base[from][to].as_bytes());
        permutations.sort_unstable();
        permutations.dedup();
        for permutation in permutations
            .into_iter()
            .map(|s| String::from_utf8(s).unwrap())
        {
            if !illegal_arrow_move(from, &permutation) {
                next[from][to] = permutation;
                ret.extend_from_slice(&calculate_all_arrow_paths_rec(
                    &next, next_from, next_to,
                ));
            }
        }
        ret
    }

    let arrow_paths = std::array::from_fn(|from| {
        std::array::from_fn(|to| ARROWS[from][to].to_string())
    });

    calculate_all_arrow_paths_rec(&arrow_paths, 0, 0)
}

fn illegal_numpad_move(c: usize, mv: &str) -> bool {
    match c {
        NUMPAD_0 => mv.starts_with('<'),
        NUMPAD_1 => mv.starts_with('v'),
        NUMPAD_2 => false,
        NUMPAD_3 => false,
        NUMPAD_4 => mv.starts_with("vv"),
        NUMPAD_5 => false,
        NUMPAD_6 => false,
        NUMPAD_7 => mv.starts_with("vvv"),
        NUMPAD_8 => false,
        NUMPAD_9 => false,
        NUMPAD_A => mv.starts_with("<<"),
        _ => unreachable!(),
    }
}

fn illegal_arrow_move(c: usize, mv: &str) -> bool {
    match c {
        ARROW_LEFT => mv.starts_with('^'),
        ARROW_UP => mv.starts_with('<'),
        ARROW_DOWN => false,
        ARROW_RIGHT => false,
        ARROW_A => mv.starts_with("<<"),
        _ => unreachable!(),
    }
}

fn permutations(s: &[u8]) -> Vec<Vec<u8>> {
    if s.len() < 2 {
        return vec![s.to_vec()];
    }
    let mut ret = vec![];
    for i in 0..s.len() {
        let mut new = s.to_vec();
        let c = new.remove(i);
        let next = permutations(&new);
        ret.extend(
            next.into_iter()
                .map(|s| s.into_iter().chain(std::iter::once(c)).collect()),
        );
    }
    ret
}

fn find_input_length(
    arrow_paths: &[[String; 5]; 5],
    number_paths: &HashMap<(usize, usize), String>,
    pairs: &[(usize, usize)],
    iterations: usize,
) -> i64 {
    let mut groups: HashMap<String, i64> = HashMap::new();
    for pair in pairs {
        *groups.entry(number_paths[pair].clone()).or_default() += 1;
    }

    for _ in 0..iterations {
        let mut new_groups = HashMap::new();
        for (arrows, count) in groups {
            for pair in format!("A{arrows}A").as_bytes().windows(2) {
                let pair: Vec<_> = pair
                    .iter()
                    .cloned()
                    .map(|c| arrow_char(char::from(c)))
                    .collect();
                let (from, to) = (pair[0], pair[1]);
                *new_groups
                    .entry(arrow_paths[from][to].clone())
                    .or_default() += count;
            }
        }
        groups = new_groups;
    }
    groups
        .iter()
        .map(|(k, v)| i64::try_from(k.len() + 1).unwrap() * v)
        .sum()
}

fn find_min_complexity(
    code: &str,
    iterations: usize,
    all_arrow_paths: &[[[String; 5]; 5]],
) -> i64 {
    let code_pairs: Vec<_> = format!("A{code}")
        .as_bytes()
        .windows(2)
        .map(|pair| {
            (
                numpad_char(char::from(pair[0])),
                numpad_char(char::from(pair[1])),
            )
        })
        .collect();
    let all_number_paths = calculate_number_paths_for(&code_pairs);
    all_arrow_paths
        .par_iter()
        .map(|arrow_paths| {
            all_number_paths
                .iter()
                .map(|number_paths| {
                    complexity(
                        find_input_length(
                            arrow_paths,
                            number_paths,
                            &code_pairs,
                            iterations,
                        ),
                        code,
                    )
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

pub fn parse(fh: File) -> Result<Vec<String>> {
    Ok(parse::raw_lines(fh).collect())
}

pub fn part1(codes: Vec<String>) -> Result<i64> {
    let all_arrow_paths = calculate_all_arrow_paths();
    Ok(codes
        .into_iter()
        .map(|code| find_min_complexity(&code, 2, &all_arrow_paths))
        .sum())
}

pub fn part2(codes: Vec<String>) -> Result<i64> {
    let all_arrow_paths = calculate_all_arrow_paths();
    Ok(codes
        .into_iter()
        .map(|code| find_min_complexity(&code, 25, &all_arrow_paths))
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 21).unwrap()).unwrap()).unwrap(),
        188398
    );
    assert_eq!(
        part2(parse(parse::data(2024, 21).unwrap()).unwrap()).unwrap(),
        230049027535970
    );
}
