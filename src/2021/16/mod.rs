struct BitIter {
    byte: u8,
    pos: u8,
}

impl BitIter {
    fn new(byte: u8) -> Self {
        Self { byte, pos: 0 }
    }
}

impl Iterator for BitIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= 8 {
            return None;
        }

        let ret = self.byte.leading_ones() > 0;
        self.byte <<= 1;
        self.pos += 1;
        Some(ret)
    }
}

fn bits(bytes: impl Iterator<Item = u8>) -> impl Iterator<Item = bool> {
    bytes.flat_map(BitIter::new)
}

struct LiteralU8(u8);

impl FromIterator<bool> for LiteralU8 {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = bool>,
    {
        let mut ret = 0;
        for b in iter {
            ret = (ret << 1) | if b { 1 } else { 0 };
        }
        Self(ret)
    }
}

struct LiteralU16(u16);

impl FromIterator<bool> for LiteralU16 {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = bool>,
    {
        let mut ret = 0;
        for b in iter {
            ret = (ret << 1) | if b { 1 } else { 0 };
        }
        Self(ret)
    }
}

pub struct Packet {
    version: u8,
    id: u8,
    contents: PacketContents,
}

enum PacketContents {
    Literal { value: u64 },
    Operator { packets: Vec<Packet> },
}

impl Packet {
    fn parse(bits: &mut impl Iterator<Item = bool>) -> (Self, usize) {
        let LiteralU8(version) = bits.take(3).collect();
        let LiteralU8(id) = bits.take(3).collect();
        let mut nbits = 6;
        let contents = if id == 4 {
            let (value, size) = read_varnum(bits.by_ref());
            nbits += size;
            PacketContents::Literal { value }
        } else {
            let mut packets = vec![];
            nbits += 1;
            if bits.next().unwrap() {
                let LiteralU16(subpacket_count) = bits.take(11).collect();
                nbits += 11;
                for _ in 0..subpacket_count {
                    let (packet, size) = Self::parse(bits);
                    packets.push(packet);
                    nbits += size;
                }
            } else {
                let LiteralU16(remaining_bits) = bits.take(15).collect();
                nbits += 15;
                let mut remaining_bits = usize::from(remaining_bits);
                while remaining_bits > 0 {
                    let (packet, size) = Self::parse(bits);
                    packets.push(packet);
                    nbits += size;
                    remaining_bits -= size;
                }
            }
            PacketContents::Operator { packets }
        };
        (
            Self {
                version,
                id,
                contents,
            },
            nbits,
        )
    }

    fn subpackets(&self) -> impl Iterator<Item = &Self> {
        let mut to_return = std::collections::VecDeque::new();
        to_return.push_back(self);
        Subpackets { to_return }
    }

    fn eval(&self) -> i64 {
        match &self.contents {
            PacketContents::Literal { value } => (*value).try_into().unwrap(),
            PacketContents::Operator { packets } => match self.id {
                0 => packets.iter().map(|packet| packet.eval()).sum(),
                1 => packets.iter().map(|packet| packet.eval()).product(),
                2 => {
                    packets.iter().map(|packet| packet.eval()).min().unwrap()
                }
                3 => {
                    packets.iter().map(|packet| packet.eval()).max().unwrap()
                }
                4 => unreachable!(),
                5 => {
                    if packets[0].eval() > packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].eval() < packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].eval() == packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn read_varnum(bits: &mut impl Iterator<Item = bool>) -> (u64, usize) {
    let mut ret = 0;
    let mut nbits = 0;
    while bits.next().unwrap() {
        let LiteralU8(chunk) = bits.take(4).collect();
        ret = (ret << 4) | u64::from(chunk);
        nbits += 5;
    }
    let LiteralU8(chunk) = bits.take(4).collect();
    ret = (ret << 4) | u64::from(chunk);
    nbits += 5;
    (ret, nbits)
}

struct Subpackets<'a> {
    to_return: std::collections::VecDeque<&'a Packet>,
}

impl<'a> Iterator for Subpackets<'a> {
    type Item = &'a Packet;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.to_return.pop_front();
        if let Some(next) = next {
            match &next.contents {
                PacketContents::Literal { .. } => {}
                PacketContents::Operator { packets } => {
                    self.to_return.extend(packets.iter());
                }
            }
        }
        next
    }
}

pub fn parse(fh: std::fs::File) -> anyhow::Result<Packet> {
    let line = crate::util::parse::lines(fh).next().unwrap();
    let mut bits = bits(line.as_bytes().chunks(2).map(|bs| {
        u8::from_str_radix(std::str::from_utf8(bs).unwrap(), 16).unwrap()
    }));
    let (packet, _) = Packet::parse(bits.by_ref());
    Ok(packet)
}

pub fn part1(packet: Packet) -> anyhow::Result<i64> {
    Ok(packet
        .subpackets()
        .map(|packet| i64::from(packet.version))
        .sum())
}

pub fn part2(packet: Packet) -> anyhow::Result<i64> {
    Ok(packet.eval())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(crate::util::data(2021, 16).unwrap()).unwrap()).unwrap(),
        979
    );
    assert_eq!(
        part2(parse(crate::util::data(2021, 16).unwrap()).unwrap()).unwrap(),
        277110354175
    );
}
