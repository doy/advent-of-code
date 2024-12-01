use advent_of_code::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Int(left) => match other {
                Self::Int(right) => left.cmp(right),
                Self::List(_) => Self::List(vec![self.clone()]).cmp(other),
            },
            Self::List(left) => match other {
                Self::Int(_) => self.cmp(&Self::List(vec![other.clone()])),
                Self::List(right) => left.cmp(right),
            },
        }
    }
}

impl Packet {
    fn parse(s: &str) -> Result<(Self, &str)> {
        if let Some(mut s) = s.strip_prefix('[') {
            let mut l = vec![];
            while !s.starts_with([',', ']']) {
                let (packet, next) = Self::parse(s)?;
                l.push(packet);
                match next.as_bytes()[0] {
                    b',' => s = &next[1..],
                    b']' => {
                        s = next;
                        break;
                    }
                    _ => bail!("failed to parse"),
                }
            }
            Ok((Self::List(l), &s[1..]))
        } else {
            let end = s.find([',', ']']).unwrap_or(s.len());
            Ok((Self::Int(s[..end].parse()?), &s[end..]))
        }
    }
}

impl std::str::FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (packet, rest) = Packet::parse(s)?;
        if !rest.is_empty() {
            bail!("trailing data: '{}'", rest);
        }
        Ok(packet)
    }
}

pub fn parse(fh: File) -> Result<Vec<(Packet, Packet)>> {
    let mut lines = parse::raw_lines(fh);

    let mut pairs = vec![];
    loop {
        let packet1 = lines.next().unwrap();
        let packet2 = lines.next().unwrap();
        pairs.push((packet1.parse()?, packet2.parse()?));
        if let Some(line) = lines.next() {
            assert_eq!(line, "");
        } else {
            break;
        }
    }
    Ok(pairs)
}

pub fn part1(pairs: Vec<(Packet, Packet)>) -> Result<usize> {
    let mut total = 0;
    for (i, (left, right)) in pairs.iter().enumerate() {
        if left <= right {
            total += i + 1;
        }
    }
    Ok(total)
}

pub fn part2(pairs: Vec<(Packet, Packet)>) -> Result<usize> {
    let div_a = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div_b = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    let mut packets: Vec<Packet> = pairs
        .into_iter()
        .flat_map(|(left, right)| {
            std::iter::once(left).chain(std::iter::once(right))
        })
        .chain(std::iter::once(div_a.clone()))
        .chain(std::iter::once(div_b.clone()))
        .collect();
    packets.sort_unstable();
    let idx_a =
        packets.iter().position(|packet| packet == &div_a).unwrap() + 1;
    let idx_b =
        packets.iter().position(|packet| packet == &div_b).unwrap() + 1;
    Ok(idx_a * idx_b)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 13).unwrap()).unwrap()).unwrap(),
        6070
    );
    assert_eq!(
        part2(parse(parse::data(2022, 13).unwrap()).unwrap()).unwrap(),
        20758
    );
}
