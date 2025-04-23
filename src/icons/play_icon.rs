use yew::prelude::*;
use crate::components::icon::icon::Icon;
use crate::icons::nav_struct::Props;

#[function_component(PlayIcon)]
pub fn play_icon(props: &Props) -> Html {
    let class_name = props.class_name.clone();
    let width = props.width.clone();
    let height = props.height.clone();
    html! {
        <Icon>
            <svg t="1745386698969" class={classes!(class_name,"icon")} viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2052" width={width} height={height}><path d="M715.8 493.5L335 165.1c-14.2-12.2-35-1.2-35 18.5v656.8c0 19.7 20.8 30.7 35 18.5l380.8-328.4c10.9-9.4 10.9-27.6 0-37z" fill={props.color.clone()} p-id="2053"></path></svg>
        </Icon>
    }
}