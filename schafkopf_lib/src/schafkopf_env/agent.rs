use crate::schafkopf_env::game_logic::{Auction, Card, Contract, PlayerGameState};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

/// An agent is a player in the game.
/// It can be a human player or a bot, but has to be able to make decisions regarding:
/// - announcing intent
/// - bidding a contract
/// - playing a card
pub trait Agent {
    /// Returns true if the player wants to announce the intent to play.
    fn get_intent(&mut self, state: &PlayerGameState, auction: &Auction) -> bool;
    /// Returns the contract the player wants to bid.
    fn get_bid(&mut self, state: &PlayerGameState, auction: &Auction) -> Contract;
    /// Returns the card the player wants to play.
    fn get_play(&mut self, state: &PlayerGameState, legal_plays: Vec<&Card>) -> Card;
}

pub struct RandomAgent {
    rng: ThreadRng,
}

impl RandomAgent {
    pub fn new() -> RandomAgent {
        RandomAgent { rng: thread_rng() }
    }
}

impl Agent for RandomAgent {
    fn get_intent(&mut self, state: &PlayerGameState, auction: &Auction) -> bool {
        self.rng.gen_bool(0.5)
    }

    fn get_bid(&mut self, state: &PlayerGameState, auction: &Auction) -> Contract {
        let valid_contracts = auction.valid_bids(Some(state.hand));
        *valid_contracts.choose(&mut self.rng).unwrap()
    }

    fn get_play(&mut self, state: &PlayerGameState, legal_plays: Vec<&Card>) -> Card {
        **legal_plays.choose(&mut self.rng).unwrap()
    }
}
