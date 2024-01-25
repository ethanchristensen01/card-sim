use rand::distributions::{Distribution, Standard};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
pub struct RankNumberMismatch();

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Distribution<Card> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card {
            suit: rng.gen(),
            rank: rng.gen(),
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = RankNumberMismatch;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Ace,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => Err(RankNumberMismatch())?,
        })
    }
}

impl Distribution<Suit> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        let index: u8 = rng.gen_range(0..4);

        match index {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Rank> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Rank {
        let index: u8 = rng.gen_range(1..14);

        Rank::try_from(index).expect("index should be in [1, 13]")
    }
}
