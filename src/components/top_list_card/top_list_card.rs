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
    
    let style = use_memo(props.color.clone(), |color| {
        if color.is_empty() {
            String::new()
        } else {
            format!("background: {color}")
        }
    });


    let hand_click = {
      let cb = props.on_click.clone();
      let title = props.title.clone();
      Callback::from(move |_:MouseEvent| {
        log::info!("clicked top list {}", title.clone());
        cb.emit(title.clone()); 
      })
    };
 
    html! {
        <li 
          style={(*style).clone()}
          class={classes!(classes)}
          onclick={hand_click}
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
