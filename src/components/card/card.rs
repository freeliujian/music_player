use yew::prelude::*;
use crate::components::card::styles::styles;
use yew_router::prelude::*;
use crate::router::SubForHomeRoute;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
  #[prop_or_default]
  pub url: String,
  #[prop_or_default]
  pub img_url: String,
  #[prop_or_default]
  pub text: String,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let classes = styles();
    let location = use_navigator().expect("Navigator not found");

    let handle_click = {
      let location = location.clone();
      Callback::from(|_:MouseEvent| {
        // location.push(url);
      })
    };
    
    html! {
        <li class={classes} onclick={handle_click}>
          <div>
            <img src={props.img_url.clone()} />
            <div class="cover" />
          </div>
          <p>
            {props.text.clone()}
          </p>
        </li>
    }
} 
