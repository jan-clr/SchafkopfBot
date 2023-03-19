use schafkopf_lib::schafkopf_env::game_logic::Card;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardCompProps {
    pub card: Card,
    #[prop_or(0)]
    pub index: usize,
}

#[function_component]
pub fn CardComp(props: &CardCompProps) -> Html {
    let mut classes = vec!["p-card"];
    if props.index == 0 {
        classes.push("p-card-first");
    }
    html! {
        <div class={classes!(classes)}>
            { format!("{}", props.card) }
        </div>
    }
}
