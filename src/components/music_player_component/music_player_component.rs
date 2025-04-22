use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::music_player_component::styles::styles;
use crate::config_provide::context::ThemeContextProvider;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub children: Html,
}

#[styled_component(MusicPlayerComponent)]
pub fn music_player_component(props: &Props) -> Html {
    let theme = use_context::<ThemeContextProvider>().expect("Theme context not available");
    let classes = styles(&*theme);
    html! {
        <div
        class={classes!(classes ,props.class_name.clone())}
        >
            {props.children.clone()}
        </div>
    }
}