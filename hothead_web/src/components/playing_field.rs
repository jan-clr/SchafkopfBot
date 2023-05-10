use crate::components::drop_zone::DropZone;
use crate::components::hand::HandComp;
use schafkopf_lib::schafkopf_env::game_logic::{Card, Suit, Value};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PlayingFieldProps {}

#[function_component]
pub fn PlayingField(props: &PlayingFieldProps) -> Html {
    let card1 = Card {
        suit: Suit::Acorns,
        value: Value::Ace,
    };
    let card2 = Card {
        suit: Suit::Bells,
        value: Value::Under,
    };
    html! {
        <div class="container">
            <DropZone />
            <HandComp cards={ vec![card1, card2] } />
        </div>
    }
}
