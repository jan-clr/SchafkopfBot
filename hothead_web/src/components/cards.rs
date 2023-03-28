use gloo::console::log;
use schafkopf_lib::schafkopf_env::game_logic::Card;
use stdweb::traits::IMouseEvent;
use stdweb::web::event::MouseDownEvent;
use web_sys::{HtmlDivElement, MouseEvent};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardCompProps {
    pub card: Card,
    #[prop_or(0)]
    pub index: usize,
}

pub enum Msg {
    DragStart,
    Drag,
}

#[function_component]
pub fn CardComp(props: &CardCompProps) -> Html {
    let mut classes = vec!["p-card"];
    let dragging_handle = use_state(|| false);
    let card_ref = use_node_ref();
    let x = use_state(|| 0);
    let y = use_state(|| 0);
    let start_x = use_state(|| 0);
    let start_y = use_state(|| 0);

    let onmousedown = {
        let dragging_handle = dragging_handle.clone();
        let x = x.clone();
        let y = y.clone();
        let node_ref = card_ref.clone();
        Callback::from(move |e: MouseEvent| {
            dragging_handle.set(true);
            log!("Drag start", *x, *y);
            if let Some(card_elem) = node_ref.cast::<HtmlDivElement>() {
                card_elem.style().set_property("transition", "0s").unwrap();
            }
        })
    };

    let onmouseup = {
        let dragging_handle = dragging_handle.clone();
        let node_ref = card_ref.clone();
        let start_x = start_x.clone();
        let start_y = start_y.clone();

        Callback::from(move |e: MouseEvent| {
            log!("Drag end");
            dragging_handle.set(false);
            if let Some(card_elem) = node_ref.cast::<HtmlDivElement>() {
                card_elem.style().remove_property("top").unwrap();
                card_elem.style().remove_property("left").unwrap();
                card_elem.style().remove_property("transition").unwrap();
            }
        })
    };
    // for some reason, if this is called onmousemove, the compiler sees it as ListenerKind::onmousemove and not as a function
    let on_mousemove = {
        let card_ref = card_ref.clone();
        let dragging_handle = dragging_handle.clone();
        let start_x = start_x.clone();
        let start_y = start_y.clone();
        Callback::from(move |e: MouseEvent| {
            let new_x = e.client_x();
            let new_y = e.client_y();
            let diff_x = new_x - *start_x;
            let diff_y = new_y - *start_y;
            x.set(new_x);
            y.set(new_y);
            //log!("Drag", *x, *y);
            if !*dragging_handle {
                start_x.set(new_x);
                start_y.set(new_y);
                return;
            }
            if let Some(card_elem) = card_ref.cast::<HtmlDivElement>() {
                let new_top = diff_y;
                let new_left = diff_x;
                log!(format!("{:?}", card_elem.style().length()));
                card_elem
                    .style()
                    .set_property("left", &format!("{}px", new_left))
                    .unwrap();
                card_elem
                    .style()
                    .set_property("top", &format!("{}px", new_top))
                    .unwrap();
            }
        })
    };
    html! {
        <div ref={card_ref} class={classes!(classes)} {onmousedown} {onmouseup} onmousemove={on_mousemove} onclick={Callback::from(move |e:MouseEvent| log!("Click"))}>
            { format!("{}", props.card) }
        </div>
    }
}
