extern crate rand;

use rand::Rng;

#[derive(PartialEq, Clone, Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    const VALUES: [Self; 4] = [Self::Heart, Self::Diamond, Self::Spade, Self::Club];

    pub fn random() -> Suit {
        Suit::VALUES[rand::thread_rng().gen_range(0,4)].clone()
    }

    pub fn translate(value: u8) -> Suit {
        Self::VALUES[(value - 1) as usize].clone()
    }
}


impl Rank {
    const VALUES: [Self; 5] = [
        Self::Ace,
        Self::Number(0),
        Self::Jack,
        Self::Queen,
        Self::King,
    ];

    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        match Self::VALUES[rng.gen_range(0,5)].clone() {
            Self::Number(_) => return Self::Number(rng.gen_range(2,11)),
            rest => return rest,
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Self::Ace,
            2..=10 => Self::Number(value),
            _ => Self::VALUES[(value-9) as usize].clone(),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}


pub fn winner_card(card: Card) -> bool {
    card.rank == Rank::Ace && card.suit == Suit::Spade
}


#[test]
fn test_winner() {
	let winner = Card {
		rank: Rank::Ace,
		suit: Suit::Spade,
	};
	for rank in 1..14 {
		for suit in 1..5 {
			let card = Card {
				rank: Rank::translate(rank),
				suit: Suit::translate(suit),
			};
			if card != winner {
				assert!(!winner_card(card));
			} else {
				assert!(winner_card(card));
			}
		}
	}
}