use strum::IntoEnumIterator;
use strum_macros::{EnumIter};
use rand::seq::SliceRandom;


#[derive(Debug)]
pub struct Game {
    trick: u8,
    pub played: Vec<Card>,
    pub hands: [Hand; 4],
    pub contract: Contract,
}

struct Dealer {
    deck: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer {
            deck: Card::deck(),
        }
    }

    pub fn deal(&mut self) -> Hand {
        if self.deck.len() < 8 {
            self.reset();
        }
        let mut hand = Hand {
            cards: Vec::new(),
            played: Vec::new()
        };

        for _ in 0..8 {
            let card = self.deck.pop().unwrap();
            hand.cards.push(card);
        }
        hand
    }

    pub fn reset(&mut self) {
        self.deck = Card::deck();
    }
}

impl Game {
    pub fn new() -> Game {
        let mut dealer = Dealer::new();
        Game {
            trick: 0,
            played: Vec::new(),
            hands: [dealer.deal(), dealer.deal(), dealer.deal(), dealer.deal()],
            contract: Contract::Call(Suit::Acorns)
        }
    }


}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub played: Vec<Card>
}

#[derive(Copy, Clone, EnumIter, Debug)]
pub enum Suit {
    Acorns,
    Leaves,
    Hearts,
    Bells
}

#[derive(Copy, Clone, EnumIter, Debug)]
pub enum Value {
    Seven,
    Eight,
    Nine,
    Ten,
    Under,
    Ober,
    King,
    Ace,
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    suit: Suit,
    value: Value
}

impl Card {
    pub fn all() -> Vec<Card> {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for value in Value::iter() {
                cards.push(Card { suit, value });
            }
        }
        return cards
    }

    pub fn deck() -> Vec<Card> {
        let mut cards = Card::all();
        cards.shuffle(&mut rand::thread_rng());
        cards
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Contract {
    Call(Suit),
    Solo(Suit),
    Wenz,
    Ramsch
}