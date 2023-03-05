use schafkopf_bot::schafkopf_env::game_state::Game;

fn main() {
    let game = Game::new();
    let player = game.get_player_game_state(0);
    println!("{}", player.hand);
}
