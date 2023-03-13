use crate::schafkopf_env::agent::{Agent, RandomAgent};
use crate::schafkopf_env::game_logic::{Auction, Game};

pub struct Match {
    pub score: [u32; 4],
    pub games: Vec<Game>,
    players: Vec<Box<dyn Agent>>,
    stopped: bool,
    next_forehand: usize,
}

impl Match {
    fn new() -> Match {
        Match {
            score: [0; 4],
            games: Vec::new(),
            players: Vec::new(),
            stopped: false,
            next_forehand: 0,
        }
    }

    fn register_player(&mut self, player: Box<dyn Agent>) {
        if self.players.len() == 4 {
            println!("Match is full");
            return;
        }
        self.players.push(player);
    }

    fn replace_player(&mut self, player: Box<dyn Agent>, index: usize) {
        if index >= self.players.len() {
            println!("Index out of bounds");
            return;
        }
        self.players[index] = player;
    }

    fn is_ready(&self) -> bool {
        self.players.len() == 4
    }

    fn play_game(&mut self) {
        if !self.is_ready() || self.stopped {
            println!("Match is not ready");
            return;
        }

        let mut game = Game::new(self.next_forehand);
        let mut player_index = Some(self.next_forehand);

        let mut auction = Auction::new(self.next_forehand);
        while !auction.bidding_phase_started() && player_index.is_some() {
            let player = game.get_player_game_state(player_index.unwrap());
            let bid = self.players[player_index.unwrap()].get_bid(&player, &auction);
            player_index = auction.next_bidder;
        }

        while !auction.is_finished() && player_index.is_some() {
            let player_state = game.get_player_game_state(player_index.unwrap());
            auction.bid(self.players[player_index.unwrap()].get_bid(&player_state, &auction));
            player_index = auction.next_bidder;
        }

        game.contract = auction.winning_contract();
        game.declarer = auction.highest_bidder;
        assert!(game.is_ready_to_play());

        while !game.is_over() && player_index.is_some() {
            let player_state = game.get_player_game_state(player_index.unwrap());
            let legal_plays = game.get_legal_actions(&game.hands[player_index.unwrap()]);
            game.play_card(
                self.players[player_index.unwrap()].get_play(&player_state, legal_plays),
            );
            player_index = Some(game.next_player);
        }
    }
}
