#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ace => 'A',
                Self::King => 'K',
                Self::Queen => 'Q',
                Self::Jack => 'J',
                Self::Ten => 'T',
                Self::Nine => '9',
                Self::Eight => '8',
                Self::Seven => '7',
                Self::Six => '6',
                Self::Five => '5',
                Self::Four => '4',
                Self::Three => '3',
                Self::Two => '2',
            }
        )
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => bail!("unknown card char {value}"),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum WildCard {
    Ace = 14,
    King = 13,
    Queen = 12,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl std::fmt::Display for WildCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ace => 'A',
                Self::King => 'K',
                Self::Queen => 'Q',
                Self::Ten => 'T',
                Self::Nine => '9',
                Self::Eight => '8',
                Self::Seven => '7',
                Self::Six => '6',
                Self::Five => '5',
                Self::Four => '4',
                Self::Three => '3',
                Self::Two => '2',
                Self::Joker => 'J',
            }
        )
    }
}

impl From<Card> for WildCard {
    fn from(value: Card) -> Self {
        match value {
            Card::Ace => Self::Ace,
            Card::King => Self::King,
            Card::Queen => Self::Queen,
            Card::Jack => Self::Joker,
            Card::Ten => Self::Ten,
            Card::Nine => Self::Nine,
            Card::Eight => Self::Eight,
            Card::Seven => Self::Seven,
            Card::Six => Self::Six,
            Card::Five => Self::Five,
            Card::Four => Self::Four,
            Card::Three => Self::Three,
            Card::Two => Self::Two,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Shape {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hand {
    cards: [Card; 5],
    bid: i64,
}

impl Hand {
    fn shape(&self) -> Shape {
        let mut card_counts: Vec<i64> = self
            .cards
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, card| {
                *map.entry(card).or_default() += 1;
                map
            })
            .into_values()
            .collect();
        card_counts.sort_unstable();
        if card_counts == [5] {
            Shape::FiveOfAKind
        } else if card_counts == [1, 4] {
            Shape::FourOfAKind
        } else if card_counts == [2, 3] {
            Shape::FullHouse
        } else if card_counts == [1, 1, 3] {
            Shape::ThreeOfAKind
        } else if card_counts == [1, 2, 2] {
            Shape::TwoPair
        } else if card_counts == [1, 1, 1, 2] {
            Shape::OnePair
        } else if card_counts == [1, 1, 1, 1, 1] {
            Shape::HighCard
        } else {
            unreachable!()
        }
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}: {}",
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4],
            self.bid
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.shape().cmp(&other.shape());
        if cmp != Ordering::Equal {
            return cmp;
        }
        for i in 0..5 {
            let cmp = self.cards[i].cmp(&other.cards[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let cards = parts.next().ok_or_else(|| anyhow!("no cards"))?;
        let cards = cards
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Card>>>()?;
        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];
        let bid = parts.next().ok_or_else(|| anyhow!("no bid"))?.parse()?;
        Ok(Hand { cards, bid })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WildHand {
    cards: [WildCard; 5],
    bid: i64,
}

impl WildHand {
    fn shape(&self) -> Shape {
        let mut card_counts: HashMap<WildCard, i64> = self
            .cards
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, card| {
                *map.entry(card).or_default() += 1;
                map
            });
        let jokers = card_counts.remove(&WildCard::Joker).unwrap_or(0);
        let mut card_counts: Vec<i64> = card_counts.into_values().collect();
        card_counts.sort_unstable();
        if card_counts.iter().max().unwrap_or(&0) + jokers == 5 {
            Shape::FiveOfAKind
        } else if card_counts.iter().max().unwrap_or(&0) + jokers == 4 {
            Shape::FourOfAKind
        } else if card_counts == [2, 3]
            || card_counts == [2, 2]
            || card_counts == [3, 1]
        {
            Shape::FullHouse
        } else if card_counts.iter().max().unwrap_or(&0) + jokers == 3 {
            Shape::ThreeOfAKind
        } else if card_counts == [1, 2, 2] {
            Shape::TwoPair
        } else if card_counts.iter().max().unwrap_or(&0) + jokers == 2 {
            Shape::OnePair
        } else if card_counts.iter().max().unwrap_or(&0) + jokers == 1 {
            Shape::HighCard
        } else {
            unreachable!()
        }
    }
}

impl std::fmt::Display for WildHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}: {}",
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4],
            self.bid
        )
    }
}

impl Ord for WildHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.shape().cmp(&other.shape());
        if cmp != Ordering::Equal {
            return cmp;
        }
        for i in 0..5 {
            let cmp = self.cards[i].cmp(&other.cards[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for WildHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Hand> for WildHand {
    fn from(value: Hand) -> Self {
        Self {
            cards: [
                value.cards[0].into(),
                value.cards[1].into(),
                value.cards[2].into(),
                value.cards[3].into(),
                value.cards[4].into(),
            ],
            bid: value.bid,
        }
    }
}

pub fn parse(fh: File) -> Result<Vec<Hand>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(mut hands: Vec<Hand>) -> Result<i64> {
    hands.sort_unstable();
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i64::try_from(i).unwrap() + 1) * hand.bid)
        .sum())
}

pub fn part2(hands: Vec<Hand>) -> Result<i64> {
    let mut hands: Vec<WildHand> =
        hands.into_iter().map(|h| h.into()).collect();
    hands.sort_unstable();
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i64::try_from(i).unwrap() + 1) * hand.bid)
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 7).unwrap()).unwrap()).unwrap(),
        253205868
    );
    assert_eq!(
        part2(parse(parse::data(2023, 7).unwrap()).unwrap()).unwrap(),
        253907829
    );
}
