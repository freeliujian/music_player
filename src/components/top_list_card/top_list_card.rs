use yew::prelude::*;
use crate::components::top_list_card::styles::styles;

#[derive(Properties, PartialEq, Clone)]
pub struct TopListCardProps {
  #[prop_or_default]
 pub on_click: Callback<String>,
 #[prop_or_default]
 pub color: String,
 #[prop_or_default]
 pub title: String,
 #[prop_or_default]
 pub content: Html,
}

#[function_component(TopListCard)]
pub fn top_list_card(props: &TopListCardProps) -> Html {
    let classes = styles();
    
    let style_color = || {
        if props.color.is_empty() {
            String::new() 
        } else {
            format!("background: {}", props.color)
        }
    };
 
    html! {
        <li 
          style={style_color()}
          class={classes!(classes)}
        >
          <h2 class={"title"}>
            {props.title.clone()}
          </h2>
          <div>
            {props.content.clone()}
          </div>
        </li>
    }
} 
