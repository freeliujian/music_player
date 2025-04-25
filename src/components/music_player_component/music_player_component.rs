use stylist::yew::styled_component;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::components::music_player_component::styles::styles;
use crate::components::player::play_progress_bar::ProgressBar;
use crate::config_provide::context::ThemeContextProvider;
use crate::icons::backward_fast_icon::BackwardIcon;
use crate::icons::forward_fast_icon::{ ForwardIcon };
use crate::icons::pause_icon::PauseIcon;
use crate::icons::retweet::RetweetIcon;
use crate::icons::play_icon::PlayIcon;
use web_sys::HtmlMediaElement;
use wasm_bindgen_futures::{spawn_local, JsFuture};


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
    let switch_pause_play = use_state(|| false);
    let audio_ref = use_node_ref();

    // let is_playing = use_state(|| false);
    let current_time= use_state(|| 0);
    let duration = use_state(|| 0);

    let on_seek = {
        let current_time = current_time.clone();
        Callback::from(move |new_time: u64| {
            current_time.set(new_time);
            log::info!("跳转到: {}秒", new_time);
        })
    };

    let on_click_play_or_pause: Callback<MouseEvent> =  {
        let switch_pause_play_click = switch_pause_play.clone();
        let audio_ref_node = audio_ref.clone();
        
        if !*switch_pause_play_click {
            Callback::from(move |_:MouseEvent| {
                let switch_pause_play_click = switch_pause_play_click.clone();
                let audio_ref_node = audio_ref_node.clone();
                if *switch_pause_play_click {
    
                }
                spawn_local(async move {
                    if let Some(audio) = audio_ref_node.cast::<HtmlMediaElement>() {
                        match audio.play() {
                            Ok(promise) => {
                                if let Err(e) = JsFuture::from(promise).await {
                                    log::error!("{:?}", &e);
                                    return
                                }
                                switch_pause_play_click.set(!*switch_pause_play_click);
                            }
                            Err(e) => {
                                log::error!("{:?}", &e);
                            }
                        }
                    }
                });
            })
        }else {
            Callback::from(move |_:MouseEvent| {
                let switch_pause_play_click = switch_pause_play_click.clone();
                let audio_ref_node = audio_ref_node.clone();
                if *switch_pause_play_click {
    
                }
                spawn_local(async move {
                    if let Some(audio) = audio_ref_node.cast::<HtmlMediaElement>() {
                        match audio.pause() {
                            Ok(_) => {
                                switch_pause_play_click.set(!*switch_pause_play_click);
                            }
                            Err(e) => {
                                log::error!("{:?}", &e);
                            }
                        }
                    }
                });
            })
        }
    };



    {
        let audio_ref_node = audio_ref.clone();
        let current_time = current_time.clone();
        let duration = duration.clone();
        use_effect_with(audio_ref_node, move |audio_ref_node: &NodeRef| {

            if let Some(audio) = audio_ref_node.cast::<HtmlMediaElement>() {
                let duration_setter = duration.clone();
                let current_time_setter = current_time.clone();
                let loaded_metadata_audio = audio.clone();
                let time_updated_audio = audio.clone();
                let loaded_metadata_cb = Closure::wrap(Box::new(move || {
                    let dur = loaded_metadata_audio.duration();
                    duration_setter.set(dur as u64);
                    log::info!("Audio duration loaded: {}", dur);
                }) as Box<dyn FnMut()>);
                audio.add_event_listener_with_callback("loadedmetadata", 
                    loaded_metadata_cb.as_ref().unchecked_ref()
                ).unwrap();

                let time_updated_cb = Closure::wrap(Box::new(move || {
                    let curr_time = time_updated_audio.current_time();
                    current_time_setter.set(curr_time as u64);
                    log::info!("Current time: {}", curr_time);
                }) as Box<dyn FnMut()>);

                audio.add_event_listener_with_callback(
                    "timeupdate", 
                    time_updated_cb.as_ref().unchecked_ref()
                ).unwrap();

                loaded_metadata_cb.forget();
                time_updated_cb.forget();
               
            }
        });
    }


    let play_main_render = {
        html! {
            <audio ref={audio_ref} id="myAudio" src="public/55 Alstroemeria Records - Bad Apple!! feat. nomico.flac"></audio>
        }
    };

    let play_or_pause_render = {
        html! {
            <div class="pause_play_content">
            {
                if *switch_pause_play {
                    html! {
                        <PauseIcon />
                    }
                }else {
                    html! {
                        <PlayIcon />
                    }
                }
            }
            </div>
        }
    };


    html! {
        <div
        class={classes!(classes ,props.class_name.clone())}
        >   
            {play_main_render}
            <div
                class="music_player_wrapper_left"
            >
                <div
                    class="music-player-play-cover"
                    onmouseover={on_mouse_over}
                    onmouseout={on_mouse_out}
                    style="background: url('public/1.jpg');background-size: contain;"
                >
                    <div class={classes!("shadow", if *show_shadow.clone() {"is_show"} else {""})}></div>
                </div>
                <div class="music-player-play-content">
                     <div class="music-player-play-name">
                        {"安河桥"}
                    </div>
                    <div class="music-player-play-author">
                        {"罗键"}
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
                    <div 
                        class={classes!("play_or_pause","music_player_btn")} 
                        onclick={on_click_play_or_pause}
                    >
                        {play_or_pause_render}
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
                        current_time={*current_time}
                        duration={*duration}
                        on_seek={on_seek}
                    />
                </div>
            </div>
            <div class="music-player-other">

            </div>
        </div>
    }
}