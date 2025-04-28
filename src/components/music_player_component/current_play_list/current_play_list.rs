use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::music_player_component::current_play_list::styles::styles;
use crate::components::music_player_component::music_player_component::CurrentPlayListVecProps;
use crate::config_provide::context::ThemeContextProvider;

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct CurrentPlayListProps {
  #[prop_or_default]
  pub name: String,
  #[prop_or_default]
  pub author: String,
  #[prop_or_default]
  pub is_share: bool,
  #[prop_or_default]
  pub time: u64,
  #[prop_or_default]
  pub url: String,
  #[prop_or_default]
  pub img: String,
  #[prop_or_default]
  pub on_click: Callback<CurrentPlayListVecProps>,
  #[prop_or_default]
  id: i32
}

#[styled_component(CurrentPlayList)]
pub fn current_play_list(props: &CurrentPlayListProps) -> Html {
  let theme = use_context::<ThemeContextProvider>().expect("Theme context not available");
    let classes = styles(&*theme);
    
    let handle_click = {
      let props = props.clone();
      Callback::from(move |_| {
          props.on_click.emit(CurrentPlayListVecProps {
            name:props.name.clone(), 
            author: props.author.clone(),
            is_share: props.is_share,
            time: props.time.clone(),
            url: props.url.clone(),
            img: props.img.clone(),
            id: props.id.clone(),
          } 
        );
      })
  };

    html! {
      <div class={classes!(classes)} onclick={handle_click}>
        <div class="music-list-content-wrapper">
          <div class="name">
          {props.name.clone()}
          </div>
          <div class="author">
              {props.author.clone()}
          </div>
          <div class="time">
              {props.time.clone()}
          </div>
        </div>
      </div>
    }
}