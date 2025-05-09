use yew::prelude::*;
use crate::components::icon::icon::Icon;
use crate::icons::nav_struct::Props;

#[function_component(ForwardIcon)]
pub fn forward_icon(props: &Props) -> Html {
    let class_name = props.class_name.clone();
    let width = props.width.clone();
    let height = props.height.clone();
    html! {
        <Icon>
            <svg t="1745386472210" class={classes!(class_name,"icon")} viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="1760"  width={width} height={height}><path d="M793.8 499.3L506.4 273.5c-10.7-8.4-26.4-0.8-26.4 12.7v451.6c0 13.5 15.7 21.1 26.4 12.7l287.4-225.8c4.1-3.2 6.2-8 6.2-12.7 0-4.7-2.1-9.5-6.2-12.7z m-320 0L186.4 273.5c-10.7-8.4-26.4-0.8-26.4 12.7v451.5c0 13.5 15.7 21.1 26.4 12.7l287.4-225.8c4.1-3.2 6.2-8 6.2-12.7 0-4.6-2.1-9.4-6.2-12.6zM857.6 248h-51.2c-3.5 0-6.4 2.7-6.4 6v516c0 3.3 2.9 6 6.4 6h51.2c3.5 0 6.4-2.7 6.4-6V254c0-3.3-2.9-6-6.4-6z"  fill={props.color.clone()} p-id="1761"></path></svg>
        </Icon>
    }
}