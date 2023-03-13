use schafkopf_lib::schafkopf_env::game_logic::Game;

fn main() {
    let game = Game::new(0);
    let player = game.get_player_game_state(0);
    println!("{}", player.hand);
}
