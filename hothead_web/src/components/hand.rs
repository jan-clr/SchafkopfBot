use crate::components::cards::CardComp;
use schafkopf_lib::schafkopf_env::game_logic::Card;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HandCompProps {
    pub cards: Vec<Card>,
}

#[function_component]
pub fn HandComp(props: &HandCompProps) -> Html {
    html! {
        <div class="hand">
            { for props.cards.iter().enumerate().map(|(idx, card)| html! { <CardComp card={ card.clone() } index={ idx as usize }/> }) }
        </div>
    }
}
