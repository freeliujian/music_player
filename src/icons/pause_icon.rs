use yew::prelude::*;
use crate::components::icon::icon::Icon;
use crate::icons::nav_struct::Props;

#[function_component(PauseIcon)]
pub fn pause_icon(props: &Props) -> Html {
    let class_name = props.class_name.clone();
    let width = props.width.clone();
    let height = props.height.clone();
    html! {
        <Icon>
            <svg t="1745386551839" class={classes!(class_name,"icon")} viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="1906" width={width} height={height}><path d="M304 176h80v672h-80zM712 176h-64c-4.4 0-8 3.6-8 8v656c0 4.4 3.6 8 8 8h64c4.4 0 8-3.6 8-8V184c0-4.4-3.6-8-8-8z" fill={props.color.clone()} p-id="1907"></path></svg>
        </Icon>
    }
}