use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DropZoneProps {}

#[function_component]
pub fn DropZone(props: &DropZoneProps) -> Html {
    html! {
        <div class="drop-zone"></div>
    }
}
