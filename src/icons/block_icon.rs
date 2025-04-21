
use yew::prelude::*;
use crate::components::icon::icon::Icon;
use crate::icons::nav_struct::Props;

#[function_component(BlockIcon)]
pub fn block_icon(props: &Props) -> Html {
    let class_name = props.class_name.clone();
    let width = props.width.clone();
    let height = props.height.clone();
    html! {
        <Icon>
            <svg t="1745208133929" class={classes!(class_name,"icon")} viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="1438" width={width} height={height}><path d="M856 376H648V168c0-8.8-7.2-16-16-16H168c-8.8 0-16 7.2-16 16v464c0 8.8 7.2 16 16 16h208v208c0 8.8 7.2 16 16 16h464c8.8 0 16-7.2 16-16V392c0-8.8-7.2-16-16-16z m-480 16v188H220V220h360v156H392c-8.8 0-16 7.2-16 16z m204 52v136H444V444h136z m224 360H444V648h188c8.8 0 16-7.2 16-16V444h156v360z" fill={props.color.clone()}  p-id="1439"></path></svg>
        </Icon>
    }
}
