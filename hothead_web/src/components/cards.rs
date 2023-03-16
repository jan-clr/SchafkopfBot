use schafkopf_lib::schafkopf_env::game_logic::Card;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardCompProps {
    pub card: Card,
}

#[function_component]
pub fn CardComp(props: &CardCompProps) -> Html {
    html! {
        <div class="card"></div>
    }
}
