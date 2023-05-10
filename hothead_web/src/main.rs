pub mod components;

use components::playing_field::PlayingField;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div >
                <PlayingField />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
