use crate::components::music_player_component::styles::styles;
use crate::components::player::play_progress_bar::ProgressBar;
use crate::config_provide::context::ThemeContextProvider;
use crate::icons::backward_fast_icon::BackwardIcon;
use crate::icons::folder_open_icon::FolderOpenIcon;
use crate::icons::forward_fast_icon::ForwardIcon;
use crate::icons::pause_icon::PauseIcon;
use crate::icons::play_icon::PlayIcon;
use crate::icons::retweet::RetweetIcon;
use stylist::yew::styled_component;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::HtmlMediaElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class_name: String,
}

struct MusicListStruct {
    name: String,
    author: String,
    is_share: bool,
    time: u64,
}

const CURRENTPLAYTITLE: &str = "当前播放";

#[styled_component(MusicPlayerComponent)]
pub fn music_player_component(props: &Props) -> Html {
    let theme = use_context::<ThemeContextProvider>().expect("Theme context not available");
    let classes = styles(&*theme);
    let show_shadow = use_state(|| false);
    let switch_pause_play = use_state(|| false);
    let show_list = use_state(|| true);

    let audio_ref = use_node_ref();

    let is_playing = use_state(|| false);
    let current_time = use_state(|| 0);
    let duration = use_state(|| 0);

    let on_mouse_out_show_shadow = show_shadow.clone();
    let on_mouse_over_show_shadow = show_shadow.clone();
    let on_mouse_out = Callback::from(move |_: MouseEvent| on_mouse_out_show_shadow.set(false));
    let on_mouse_over = Callback::from(move |_: MouseEvent| {
        on_mouse_over_show_shadow.set(true);
    });

    let music_list = use_state(|| {
        vec![MusicListStruct {
            name: String::from("安河桥"),
            author: String::from("luojian"),
            is_share: false,
            time: 5000,
        }]
    });

    let music_num = 1;

    let on_seek = {
        let current_time = current_time.clone();
        let audio_ref_node = audio_ref.clone();
        Callback::from(move |new_time: u64| {
            let audio_ref_node = audio_ref_node.clone();
            spawn_local(seek_and_play(
                new_time,
                audio_ref_node.clone(),
                current_time.clone(),
            ));
        })
    };

    async fn seek_and_play(
        new_time: u64,
        audio_ref_node: NodeRef,
        current_time: UseStateHandle<u64>,
    ) {
        if let Some(audio) = audio_ref_node.cast::<HtmlMediaElement>() {
            audio.set_current_time(new_time as f64);
            current_time.set(new_time);
            match audio.play() {
                Ok(promise) => {
                    if let Err(e) = JsFuture::from(promise).await {
                        log::error!("{:?}", &e);
                    }
                }
                Err(e) => log::error!("{:?}", &e),
            }
            log::info!("跳转到: {}秒", new_time);
        }
    }

    let on_click_play_or_pause: Callback<MouseEvent> = {
        let switch_pause_play_click = switch_pause_play.clone();
        let audio_ref_node = audio_ref.clone();

        if !*switch_pause_play_click {
            Callback::from(move |_: MouseEvent| {
                let switch_pause_play_click = switch_pause_play_click.clone();
                let audio_ref_node = audio_ref_node.clone();
                if *switch_pause_play_click {}
                spawn_local(async move {
                    if let Some(audio) = audio_ref_node.cast::<HtmlMediaElement>() {
                        match audio.play() {
                            Ok(promise) => {
                                if let Err(e) = JsFuture::from(promise).await {
                                    log::error!("{:?}", &e);
                                    return;
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
        } else {
            Callback::from(move |_: MouseEvent| {
                let switch_pause_play_click = switch_pause_play_click.clone();
                let audio_ref_node = audio_ref_node.clone();
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
                let is_play_audio = is_playing.clone();
                let loaded_metadata_cb = Closure::wrap(Box::new(move || {
                    let dur = loaded_metadata_audio.duration();
                    duration_setter.set(dur as u64);
                    log::info!("Audio duration loaded: {}", dur);
                }) as Box<dyn FnMut()>);
                audio
                    .add_event_listener_with_callback(
                        "loadedmetadata",
                        loaded_metadata_cb.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                let time_updated_cb = Closure::wrap(Box::new(move || {
                    let curr_time = time_updated_audio.current_time();
                    current_time_setter.set(curr_time as u64);
                    log::info!("Current time: {}", curr_time);
                }) as Box<dyn FnMut()>);

                let is_play_cb = Closure::wrap(Box::new(move || {
                    is_play_audio.set(true);
                }) as Box<dyn FnMut()>);

                audio
                    .add_event_listener_with_callback("play", is_play_cb.as_ref().unchecked_ref())
                    .expect("play status is error");

                audio
                    .add_event_listener_with_callback(
                        "timeupdate",
                        time_updated_cb.as_ref().unchecked_ref(),
                    )
                    .expect("timeupdate status is error");

                // audio.add_event_listener_with_callback("play", listener)

                loaded_metadata_cb.forget();
                time_updated_cb.forget();
            }
        });
    }

    let on_end_ed = {
        let audio_ref_node = audio_ref.clone();
        let audio_current_time = current_time.clone();
        let show_play = switch_pause_play.clone();
        Callback::from(move |_| {
            if let Some(audio) = audio_ref_node.cast::<HtmlMediaElement>() {
                audio.set_current_time(0.0);
                audio_current_time.set(0);
                show_play.set(false);
            }
        })
    };

    let play_main_render = {
        html! {
            <audio ref={audio_ref} id="myAudio" src="public/55 Alstroemeria Records - Bad Apple!! feat. nomico.flac" onended={on_end_ed}></audio>
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

    let handle_click_show_list = {
        let show_list = show_list.clone();
        Callback::from(move |_| {
            log::info!("{:?}", *show_list);
            show_list.set(!*show_list);
        })
    };

    let show_music_list_title_render = {
        html! {
            <>
                <div class="music-list-title">
                {CURRENTPLAYTITLE}
                </div>
                <div class="music-list-subtitle">
                    <span class="total-number">
                        {format!("总{}首", music_num)}
                    </span>
                    <div class="music-list-action">
                        <span class="save-all">
                        {"收藏全部"}
                        </span>
                        <span class="clear-list">
                            {"清空列表"}
                        </span>
                    </div>
                </div>
            </>
        }
    };

    let show_music_list_content_render = {
        html! {
            <div class="music-list-content">

            </div>
        }
    };

    let show_music_list_render = {
        html! {
            <div class="music-list-wrapper">
                {show_music_list_title_render}
                {show_music_list_content_render}
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
                <span class="open-icon"  onclick={handle_click_show_list}>
                    <FolderOpenIcon/>
                </span>
            </div>
            {
                if *show_list {
                    show_music_list_render
                }else {
                    html! {
                        <>{" "}</>
                    }
                }
            }
        </div>
    }
}
