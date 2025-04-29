use yew::prelude::*;
use crate::components::card::styles::styles;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {

}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let classes = styles();
    html! {
        <div class={classes}>
          {"卡片组件"}
        </div>
    }
} 
