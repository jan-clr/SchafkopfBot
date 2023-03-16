pub mod components;

use components::cards::CardComp;
use schafkopf_lib::schafkopf_env::game_logic::{Card, Suit, Value};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            //<cardcomp card={ Card { suit: Suit::Acorns, value: Value::Ace} } />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    let card = Card {
        suit: Suit::Acorns,
        value: Value::Ace,
    };
}
