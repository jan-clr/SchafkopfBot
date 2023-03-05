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
    None,
}

#[derive(Debug)]
pub struct Game {
    trick: u8,
    ran_away: bool,
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
            ran_away: false,
            next_player: 0,
            played: Vec::new(),
            hands: [dealer.deal(), dealer.deal(), dealer.deal(), dealer.deal()],
            contract: Contract::Call(Suit::Acorns),
        }
    }

    pub fn get_player_game_state(&self, player_nr: u8) -> PlayerGameState {
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

    /// Checks if the given action is valid for the current game state and player hand.
    /// If player hand is unknown, pass None.
    /// In this case, almost all actions have to be considered valid and can only be checked after the game is over.
    pub fn action_is_valid(&self, action: &Card, hand: Option<&Hand>) -> bool {
        let trick_cards = self
            .played
            .iter()
            .skip((self.trick * 4) as usize)
            .take(4)
            .collect::<Vec<&Card>>();
        let player_is_called = match self.contract {
            Contract::Call(suit) => {
                return if hand.is_none() {
                    false
                } else {
                    let hand = hand.unwrap();
                    action.suit == suit
                        && hand
                            .cards
                            .iter()
                            .any(|c| c.suit == suit && c.value == Value::Ace)
                }
            }
            _ => false,
        };
        let can_run = match self.contract {
            Contract::Call(suit) => {
                return if hand.is_none() {
                    false
                } else {
                    let hand = hand.unwrap();
                    return action.suit == suit
                        && hand.cards.iter().filter(|c| c.suit == suit).count() >= 4;
                }
            }
            _ => false,
        };
        let called_suit = match self.contract {
            Contract::Call(suit) => Some(suit),
            _ => None,
        };

        // check leading card validity
        if trick_cards.len() == 0 {
            if player_is_called {
                if Some(action.suit) == called_suit && action.value == Value::Ace && !self.ran_away
                {
                    // player is called and has not run away -> can't lead with called ace
                    return false;
                }
                if Some(action.suit) == called_suit && !self.ran_away && !can_run {
                    // player is called and hasn't and isn't able to run away -> can't lead with called suit
                    return false;
                }
                // card is not in called suit -> can lead with it
                // player can run away -> can lead with anything but called ace
                // player has run away -> can lead with anything
                return true;
            }
        }

        let leading_suit = trick_cards[0].suit;
        let lead_is_trump = is_trump(trick_cards[0], &self.contract);

        let has_leading_suit =
            hand.is_some() && hand.unwrap().cards.iter().any(|c| c.suit == leading_suit);
        let has_trump = hand.is_some()
            && hand
                .unwrap()
                .cards
                .iter()
                .any(|c| is_trump(c, &self.contract));
        let leading_suit_is_called = Some(leading_suit) == called_suit;

        // check non leading card validity
        return if lead_is_trump && has_trump {
            // lead is trump -> can only play trump
            is_trump(action, &self.contract)
        } else if leading_suit_is_called && player_is_called && !self.ran_away {
            // player is called and leading suit is called -> must play called ace unless ran away already
            action.suit == leading_suit && action.value == Value::Ace
        } else if has_leading_suit {
            // player has leading suit -> must play leading suit
            action.suit == leading_suit
        } else {
            // player doesn't have leading suit -> can play anything
            true
        };
    }

    pub fn get_valid_actions<'a>(&'a self, hand: &'a Hand) -> Vec<&Card> {
        hand.cards
            .iter()
            .filter(|c| self.action_is_valid(c, Some(hand)))
            .collect()
    }

    pub fn play_card(&mut self, card: Card) {
        self.played.push(card);
        self.hands[self.next_player as usize].played.push(card);
        self.hands[self.next_player as usize]
            .cards
            .retain(|c| c != &card);
        self.update_next_player();
    }

    pub fn is_ready_to_play(&self) -> bool {
        self.played.len() == 0
            && self.contract != Contract::None
            && self.trick == 0
            && self
                .hands
                .iter()
                .all(|h| h.cards.len() == 8 && h.played.len() == 0)
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
        Contract::None => false,
    }
}

#[derive(Debug)]
pub struct Auction {
    pub highest_bid: Contract,
    pub highest_bidder: u8,
    pub next_bidder: Option<u8>,
    pub intent: [bool; 4],
    intent_count: u8,
}

impl Auction {
    pub fn new(starting_bidder: u8) -> Auction {
        Auction {
            highest_bid: Contract::None,
            highest_bidder: starting_bidder,
            next_bidder: Some(starting_bidder),
            intent: [false; 4],
            intent_count: 0,
        }
    }

    pub fn valid_bids(&self) -> Vec<Contract> {
        let mut bids = vec![
            Contract::Call(Suit::Acorns),
            Contract::Call(Suit::Bells),
            Contract::Call(Suit::Leaves),
            Contract::Call(Suit::Hearts),
            Contract::Solo(Suit::Acorns),
            Contract::Solo(Suit::Bells),
            Contract::Solo(Suit::Leaves),
            Contract::Solo(Suit::Hearts),
            Contract::Wenz,
        ];
        match self.highest_bid {
            Contract::Call(_) => {
                // if highest bid is a call -> remove all calls
                bids.retain(|c| match c {
                    Contract::Call(_) => false,
                    _ => true,
                });
                bids.push(Contract::None);
            }
            Contract::Solo(_) => {
                // if highest bid is a solo -> no bids possible
                bids.clear();
            }
            Contract::Wenz => {
                // if highest bid is a wenz -> remove wenz and calls
                bids.retain(|c| match c {
                    Contract::Wenz | Contract::Call(_) => false,
                    _ => true,
                });
                bids.push(Contract::None);
            }
            // if highest bid is none -> no bids possible
            _ => {}
        }

        bids
    }

    pub fn announce_intent(&mut self, intent: bool) {
        if self.intent_count == 4 {
            return;
        }
        self.intent[self.next_bidder.expect("Bidder must exist at this point.") as usize] = intent;
        self.intent_count += 1;
        self.update_next_bidder();
    }

    fn update_next_bidder(&mut self) {
        if self.is_finished() {
            self.next_bidder = None;
        } else if self.intent_count < 4 {
            self.next_bidder =
                Some((self.next_bidder.expect("Bidder must exist at this point.") + 1) % 4);
        } else {
            let nr_bidders = self.intent.iter().filter(|i| **i).count();
            if nr_bidders == 0 {
                // no one wants to bid -> next bidder is none
                self.next_bidder = None;
                return;
            }
            if nr_bidders == 1 && self.highest_bid != Contract::None {
                // contract declared and no contenders -> next bidder is none
                self.next_bidder = None;
                return;
            }

            // skip to next player with intent to bid
            loop {
                self.next_bidder =
                    Some((self.next_bidder.expect("Bidder must exist at this point.") + 1) % 4);
                if self.intent[self.next_bidder.expect("Bidder must exist at this point.") as usize]
                {
                    break;
                }
            }
        }
    }

    pub fn bid(&mut self, bid: Contract) {
        assert!(self.valid_bids().contains(&bid));
        if let Contract::None = bid {
            self.intent[self.next_bidder.expect("Bidder must exist at this point.") as usize] =
                false;
        } else {
            self.highest_bid = bid;
            self.highest_bidder = self.next_bidder.expect("Bidder must exist at this point.");
        }
        self.update_next_bidder();
    }

    // auction is finished if either all players have passed or no higher bid is possible
    pub fn is_finished(&self) -> bool {
        self.valid_bids().is_empty() || self.next_bidder.is_none()
    }

    pub fn winning_contract(&self) -> Contract {
        if self.is_finished() {
            if let Contract::None = self.highest_bid {
                Contract::Ramsch
            } else {
                self.highest_bid
            }
        } else {
            Contract::None
        }
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

    #[test]
    fn test_auction_one_bidder() {
        let mut auction = Auction::new(0);
        auction.announce_intent(false);
        auction.announce_intent(false);
        auction.announce_intent(false);
        auction.announce_intent(true);
        assert!(!auction.is_finished());
        auction.bid(Contract::Call(Suit::Acorns));
        assert_eq!(auction.winning_contract(), Contract::Call(Suit::Acorns));
        assert_eq!(auction.highest_bidder, 3);
    }

    #[test]
    fn test_auction_options_after_call_bid() {
        let mut auction = Auction::new(0);
        auction.announce_intent(false);
        auction.announce_intent(false);
        auction.announce_intent(true);
        auction.announce_intent(true);
        auction.bid(Contract::Call(Suit::Acorns));
        assert_eq!(
            auction.valid_bids(),
            vec![
                Contract::Solo(Suit::Acorns),
                Contract::Solo(Suit::Bells),
                Contract::Solo(Suit::Leaves),
                Contract::Solo(Suit::Hearts),
                Contract::Wenz,
                Contract::None,
            ]
        );
    }

    #[test]
    fn test_auction_options_after_solo_bid() {
        let mut auction = Auction::new(0);
        auction.announce_intent(false);
        auction.announce_intent(false);
        auction.announce_intent(true);
        auction.announce_intent(true);
        auction.bid(Contract::Call(Suit::Acorns));
        auction.bid(Contract::Solo(Suit::Acorns));
        assert!(auction.valid_bids().is_empty(),);
    }

    use proptest::prelude::*;
    proptest! {

        #![proptest_config(ProptestConfig::with_cases(4))]
        #[test]
        fn test_auction_no_bids(sb in 0..4 as u8) {
            println!("sb: {}", sb);
            let mut auction = Auction::new(sb);
            auction.announce_intent(false);
            auction.announce_intent(false);
            auction.announce_intent(false);
            auction.announce_intent(false);
            assert!(auction.is_finished());
            assert_eq!(auction.winning_contract(), Contract::Ramsch);
            assert_eq!(auction.highest_bidder, sb)
        }
    }
}
