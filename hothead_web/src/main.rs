pub mod components;

use components::cards::CardComp;
use components::hand::HandComp;
use schafkopf_lib::schafkopf_env::game_logic::{Card, Suit, Value};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let card1 = Card {
        suit: Suit::Acorns,
        value: Value::Ace,
    };
    let card2 = Card {
        suit: Suit::Bells,
        value: Value::Under,
    };
    html! {
        <>
            <div class="container">
                <HandComp cards={ vec![card1, card2] } />
            </div>
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
