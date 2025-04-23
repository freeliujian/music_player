use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::music_player_component::styles::styles;
use crate::components::player::play_progress_bar::ProgressBar;
use crate::config_provide::context::ThemeContextProvider;
use crate::icons::backward_fast_icon::BackwardIcon;
use crate::icons::forward_fast_icon::{ForwardIcon};
use crate::icons::pause_icon::PauseIcon;
use crate::icons::retweet::RetweetIcon;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class_name: String,
}

#[styled_component(MusicPlayerComponent)]
pub fn music_player_component(props: &Props) -> Html {
    let theme = use_context::<ThemeContextProvider>().expect("Theme context not available");
    let classes = styles(&*theme);
    let show_shadow = use_state(|| false);
    let on_mouse_out_show_shadow = show_shadow.clone();
    let on_mouse_out = Callback::from(move |_:MouseEvent| on_mouse_out_show_shadow.set(false));
    let on_mouse_over_show_shadow = show_shadow.clone();
    let on_mouse_over = Callback::from(move |_:MouseEvent| {
        on_mouse_over_show_shadow.set(true);
    });

    html! {
        <div
        class={classes!(classes ,props.class_name.clone())}
        >
            <div
                class="music_player_wrapper_left"
            >
                <div
                    class="music-player-play-cover"
                    onmouseover={on_mouse_over}
                    onmouseout={on_mouse_out}
                >
                    <div class={classes!("shadow", if *show_shadow.clone() {"is_show"} else {""})}></div>
                </div>
                <div class="music-player-play-content">
                     <div class="music-player-play-name">
                        {"安河桥"}
                    </div>
                    <div class="music-player-play-author">
                        {"罗健"}
                    </div>
                </div>
            </div>
            <div class="music-player-play-wrapper">
                <div class="music-player-play-action">
                    <div class={classes!("retweet","music_player_btn")}>
                        <RetweetIcon/>
                    </div>
                    <div class={classes!("previous","music_player_btn")}>
                        <BackwardIcon/>
                    </div>
                    <div class={classes!("play_or_pause","music_player_btn")}>
                        <PauseIcon/>
                    </div>
                    <div class={classes!("next","music_player_btn")}>
                        <ForwardIcon/>
                    </div>
                    <div class={classes!("word","music_player_btn")}>
                        {"词"}
                    </div>
                </div>
                <div class="music-player-play-main">
                    <ProgressBar
                        current_time={11}
                        duration={11}
                        on_seek={|_| ()}
                    />
                </div>
            </div>
            <div class="music-player-other">

            </div>
        </div>
    }
}