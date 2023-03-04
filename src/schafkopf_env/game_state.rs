use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub played: Vec<Card>,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut cards = self.cards.clone();
        cards.sort_by(|a, b| {
            let mut cmp = a.suit.cmp(&b.suit);
            if cmp == Ordering::Equal {
                cmp = a.value.cmp(&b.value);
            }
            cmp
        });
        let cards = cards
            .iter()
            .rev()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        write!(f, "{}", cards.join("\n"))
    }
}

#[derive(Copy, Clone, EnumIter, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Suit {
    Bells,
    Hearts,
    Leaves,
    Acorns,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Acorns => write!(f, "🌰"),
            Suit::Leaves => write!(f, "🍀"),
            Suit::Hearts => write!(f, "❤️"),
            Suit::Bells => write!(f, "🔔"),
        }
    }
}

#[derive(Copy, Clone, EnumIter, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Value {
    Seven,
    Eight,
    Nine,
    King,
    Ten,
    Under,
    Ober,
    Ace,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
            Value::Ten => write!(f, "10"),
            Value::Under => write!(f, "U"),
            Value::Ober => write!(f, "O"),
            Value::King => write!(f, "K"),
            Value::Ace => write!(f, "A"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    pub fn all() -> Vec<Card> {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for value in Value::iter() {
                cards.push(Card { suit, value });
            }
        }
        return cards;
    }

    pub fn deck() -> Vec<Card> {
        let mut cards = Card::all();
        cards.shuffle(&mut rand::thread_rng());
        cards
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.suit, self.value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Contract {
    Call(Suit),
    Solo(Suit),
    Wenz,
    Ramsch,
}

#[derive(Debug)]
pub struct Game {
    trick: u8,
    pub next_player: u8,
    pub played: Vec<Card>,
    pub hands: [Hand; 4],
    pub contract: Contract,
}

impl Game {
    pub fn new() -> Game {
        let mut dealer = Dealer::new();
        Game {
            trick: 0,
            next_player: 0,
            played: Vec::new(),
            hands: [dealer.deal(), dealer.deal(), dealer.deal(), dealer.deal()],
            contract: Contract::Call(Suit::Acorns),
        }
    }

    pub fn player_state(&self, player_nr: u8) -> PlayerGameState {
        PlayerGameState {
            hand: &self.hands[player_nr as usize],
            contract: self.contract,
            player_nr,
            trick: &self.trick,
            played: &self.played,
        }
    }

    fn update_trick(&mut self) {
        self.trick = self.played.len() as u8 / 4;
    }

    fn determine_trick_winner(&self, trick: u8) -> Option<u8> {
        let trick_cards = self.played.iter().skip((trick * 4) as usize).take(4);

        if trick_cards.len() < 4 {
            return None;
        }

        let trick_cards = trick_cards.collect::<Vec<&Card>>();
        let leading_suit = &trick_cards[0].suit;

        let winner = trick_cards
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                let (a_is_trump, b_is_trump) =
                    (is_trump(a, &self.contract), is_trump(b, &self.contract));
                let (a_is_lead_suit, b_is_lead_suit) =
                    (a.suit == *leading_suit, b.suit == *leading_suit);
                if a_is_trump && !b_is_trump {
                    // trump wins over non-trump
                    return Ordering::Greater;
                } else if !a_is_trump && b_is_trump {
                    // trump wins over non-trump
                    return Ordering::Less;
                } else if a_is_trump && b_is_trump {
                    // Ober and Under win against numbered trumps
                    let mut cmp = a.value.cmp(&b.value);
                    if cmp == Ordering::Equal {
                        cmp = a.suit.cmp(&b.suit);
                    }
                    return cmp;
                // neither cards are trumps
                } else if a_is_lead_suit && !b_is_lead_suit {
                    // lead suit wins over non-lead suit
                    return Ordering::Greater;
                } else if !a_is_lead_suit && b_is_lead_suit {
                    // lead suit wins over non-lead suit
                    return Ordering::Less;
                } else if a_is_lead_suit && b_is_lead_suit {
                    // higher value wins if both are lead suit
                    return a.value.cmp(&b.value);
                }
                return Ordering::Equal;
            })
            .map(|(i, _)| i as u8);

        winner
    }

    fn update_next_player(&mut self) {
        if self.played.len() % 4 == 0 {
            let winner = self.determine_trick_winner(self.trick);
            if winner.is_some() {
                self.next_player = winner.unwrap();
            }
        } else {
            self.next_player = (self.next_player + 1) % 4;
        }
    }

    pub fn action_is_valid(&self, action: &Card, hand: &Hand) -> bool {
        let trick_cards = self
            .played
            .iter()
            .skip((self.trick * 4) as usize)
            .take(4)
            .collect::<Vec<&Card>>();
        let contract = self.contract;
        if trick_cards.len() == 0 {
            match contract {
                Contract::Call(suit) => {
                    if action.suit == suit
                        && hand
                            .cards
                            .iter()
                            .any(|c| c.suit == suit && c.value == Value::Ace)
                        && hand.cards.iter().filter(|c| c.suit == suit).count() < 4
                    {
                        return false;
                    }
                    todo!("Implement Laufsau rules")
                }
                _ => return true,
            }
        }
        let leading_suit = trick_cards[0].suit;
        let lead_is_trump = is_trump(trick_cards[0], &self.contract);

        false
    }
}

struct Dealer {
    deck: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer { deck: Card::deck() }
    }

    pub fn deal(&mut self) -> Hand {
        if self.deck.len() < 8 {
            self.reset();
        }
        let mut hand = Hand {
            cards: Vec::new(),
            played: Vec::new(),
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

#[derive(Debug)]
pub struct PlayerGameState<'a> {
    pub hand: &'a Hand,
    pub contract: Contract,
    pub player_nr: u8,
    pub trick: &'a u8,
    pub played: &'a Vec<Card>,
}

fn is_trump(card: &Card, contract: &Contract) -> bool {
    match contract {
        Contract::Call(_) | Contract::Ramsch => {
            card.suit == Suit::Hearts || card.value == Value::Ober || card.value == Value::Under
        }
        Contract::Solo(suit) => {
            card.suit == *suit || card.value == Value::Ober || card.value == Value::Under
        }
        Contract::Wenz => card.value == Value::Under,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck() {
        let deck = Card::deck();
        assert_eq!(deck.len(), 32);
        assert_eq!(deck.iter().filter(|c| c.suit == Suit::Acorns).count(), 8);
        assert_eq!(deck.iter().filter(|c| c.suit == Suit::Bells).count(), 8);
        assert_eq!(deck.iter().filter(|c| c.suit == Suit::Leaves).count(), 8);
        assert_eq!(deck.iter().filter(|c| c.suit == Suit::Hearts).count(), 8);
    }

    #[test]
    fn test_deal() {
        let mut dealer = Dealer::new();
        let hand = dealer.deal();
        assert_eq!(hand.cards.len(), 8);
        assert_eq!(hand.played.len(), 0);
    }

    #[test]
    fn test_deal_all() {
        let mut dealer = Dealer::new();
        for _ in 0..4 {
            dealer.deal();
        }
        assert_eq!(dealer.deck.len(), 0);
    }

    #[test]
    fn test_deal_reset() {
        let mut dealer = Dealer::new();
        for _ in 0..4 {
            dealer.deal();
        }
        assert_eq!(dealer.deck.len(), 0);
        dealer.reset();
        assert_eq!(dealer.deck.len(), 32);
    }

    #[test]
    fn test_game_new() {
        let game = Game::new();
        assert_eq!(game.trick, 0);
        assert_eq!(game.next_player, 0);
        assert_eq!(game.played.len(), 0);
        assert_eq!(game.hands.len(), 4);
        assert_eq!(game.hands[0].cards.len(), 8);
        assert_eq!(game.hands[1].cards.len(), 8);
        assert_eq!(game.hands[2].cards.len(), 8);
        assert_eq!(game.hands[3].cards.len(), 8);
    }

    #[test]
    fn test_call_matches_any() {
        let contract = Contract::Call(Suit::Acorns);
        assert!(matches!(contract, Contract::Call(_)));
    }
}
