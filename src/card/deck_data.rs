use super::card_data::{Card, Rank, Suit};
use rand::{rngs::ThreadRng, seq::SliceRandom};
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn draw_random<const N: usize>(&self, rng: &mut ThreadRng) -> [Option<Card>; N] {
        let mut buf: [Option<Card>; N] = [None; N];
        for (card, buf_slot) in self.0.choose_multiple(rng, N).copied().zip(buf.iter_mut()) {
            let _ = buf_slot.insert(card);
        }
        buf
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut deck = Vec::new();
        deck.reserve_exact(52);
        for i in 1..14 {
            deck.push(Card {
                suit: Suit::Clubs,
                rank: Rank::try_from(i).unwrap()
            });
            deck.push(Card {
                suit: Suit::Diamonds,
                rank: Rank::try_from(i).unwrap()
            });
            deck.push(Card {
                suit: Suit::Hearts,
                rank: Rank::try_from(i).unwrap()
            });
            deck.push(Card {
                suit: Suit::Spades,
                rank: Rank::try_from(i).unwrap()
            });
        }
        Self(deck)
    }
}

// impl IntoIterator for Deck {
//     type Item = Card;

//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl<'a> IntoIterator for &'a Deck {
//     type Item = &'a Card;
    
//     type IntoIter = std::slice::Iter<'a, Card>;
    
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter()
//     }
// }

// impl<'a> IntoIterator for &'a mut Deck {
//     type Item = &'a mut Card;
    
//     type IntoIter = std::slice::IterMut<'a, Card>;
    
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter_mut()
//     }
// }

// impl FromIterator<Card> for Deck {
//     fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
//         Deck(iter.into_iter().collect())
//     }
// }
