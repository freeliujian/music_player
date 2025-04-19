use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::icon::styles::styles;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub children: Html,
}

#[styled_component(Icon)]
pub fn icon_component(props: &Props) -> Html {
    let classes = styles();
    html! {
        <div
        class={classes!(classes ,props.class_name.clone())}
        >
            {props.children.clone()}
        </div>
    }
}