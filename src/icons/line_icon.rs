
use yew::prelude::*;
use crate::components::icon::icon::Icon;
use crate::icons::nav_struct::Props;

#[function_component(LineIcon)]
pub fn line_icon(props: &Props) -> Html {
    let class_name = props.class_name.clone();
    let width = props.width.clone();
    let height = props.height.clone();
    html! {
        <Icon>
            <svg t="1745155946383"  class={classes!(class_name,"icon")} viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="1554" width={width} height={height}><path d="M872 474H152c-4.4 0-8 3.6-8 8v60c0 4.4 3.6 8 8 8h720c4.4 0 8-3.6 8-8v-60c0-4.4-3.6-8-8-8z" fill={props.color.clone()} p-id="1555"></path></svg>
        </Icon>
    }
}
