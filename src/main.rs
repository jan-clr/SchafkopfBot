use schafkopf_bot::schafkopf_env::game_state::Game;

fn main() {
    let game = Game::new();
    let player = game.player_state(0);
    println!("{}", player.hand);
}
