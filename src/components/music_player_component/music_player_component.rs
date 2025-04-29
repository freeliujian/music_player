use crate::components::music_player_component::styles::styles;
use crate::components::player::play_progress_bar::ProgressBar;
use crate::config_provide::context::ThemeContextProvider;
use crate::icons::backward_fast_icon::BackwardIcon;
use crate::icons::folder_open_icon::FolderOpenIcon;
use crate::icons::forward_fast_icon::ForwardIcon;
use crate::icons::pause_icon::PauseIcon;
use crate::icons::play_icon::PlayIcon;
use crate::icons::retweet::RetweetIcon;
use crate::components::music_player_component::current_play_list::current_play_list::{CurrentPlayListProps, CurrentPlayList};
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

#[derive(Debug, Clone)]
pub struct CurrentPlayListVecProps {
    pub name: String,
    pub author: String,
    pub is_share: bool,
    pub time: u64,
    pub url: String,
    pub img: String,
    pub id: i32,
}

#[styled_component(MusicPlayerComponent)]
pub fn music_player_component(props: &Props) -> Html {
    let theme = use_context::<ThemeContextProvider>().expect("Theme context not available");
    let classes = styles(&*theme);
    let show_shadow = use_state(|| false);
    let switch_pause_play = use_state(|| false);
    let show_list = use_state(|| false);

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
        vec![
            CurrentPlayListVecProps {
                name: String::from("bad apple"),
                author: String::from("luojian"),
                is_share: false,
                time: 5000,
                url: String::from("http://localhost.:1420/public/55 Alstroemeria Records - Bad Apple!! feat. nomico.flac"),
                img: String::from("http://localhost.:1420/public/1.jpg"),
                id: 0
            },
            CurrentPlayListVecProps {
                name: String::from("倾城"),
                author: String::from("许美静"),
                is_share: false,
                time: 5000,
                url: String::from("http://localhost.:1420/public/许美静 - 倾城.mp3"),
                img: String::from("http://localhost.:1420/public/1.jpg"),
                id: 1
            },
            CurrentPlayListVecProps {
                name: String::from("张卫健"),
                author: String::from("天外有天"),
                is_share: false,
                time: 5000,
                url: String::from("http://localhost.:1420/public/张卫健 - 天外有天.mp3"),
                img: String::from("http://localhost.:1420/public/1.jpg"),
                id: 2
            },
        ]
    });
    let current_selected_play_message = use_state(|| 0);
    let play_message = use_state(|| music_list[0].clone());
    let music_num = music_list.len();

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


                loaded_metadata_cb.forget();
                time_updated_cb.forget();
            }
        });
    }


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

    let remove_music_list_click = {
        let music_list: UseStateHandle<Vec<CurrentPlayListVecProps>> = music_list.clone();
        let on_click_play_or_pause = on_click_play_or_pause.clone();
        Callback::from(move |e| {
            music_list.set(Vec::new());
            on_click_play_or_pause.emit(e);
        })
    };

    let show_music_list_title_render = {
        html! {
            <>
                <div class="music-list-title">
                {"当前播放"}
                </div>
                <div class="music-list-subtitle">
                    <span class="total-number">
                        {format!("总{}首", music_num)}
                    </span>
                    <div class="music-list-action">
                        <span class="save-all">
                        {"收藏全部"}
                        </span>
                        <span class="clear-list" onclick={remove_music_list_click}>
                            {"清空列表"}
                        </span>
                    </div>
                </div>
            </>
        }
    };

    let get_current_list_item = {
        let play_message = play_message.clone();
        let switch_pause_play = switch_pause_play.clone();
        let current_selected = current_selected_play_message.clone();
        let current_play_index = current_selected_play_message.clone();
        let music_list = music_list.clone();
        let audio_ref_node = audio_ref.clone();
        Callback::from(move |value: CurrentPlayListVecProps| {
            let audio_ref_node = audio_ref_node.clone();
            let value_clone = value.clone();
            play_message.set(value.clone());
            current_selected.set(value_clone.id);
            switch_pause_play.set(true);
            
            // 更新当前播放索引
            if let Some(index) = music_list.iter().position(|x| x.id == value.id) {
                current_play_index.set(index as i32);
            }
            
            spawn_local(async move {
                if let Some(audio) = audio_ref_node.cast::<HtmlMediaElement>() {
                    match audio.play() {
                        Ok(promise) => {
                            if let Err(e) = JsFuture::from(promise).await {
                                log::error!("{:?}", &e);
                            }
                        }
                        Err(e) => {
                            log::info!("{:?}",&e);
                        }
                    }
                }
            });
        })
    };

    let show_music_list_render = {
        html! {
            <div class="music-list-wrapper">
                {show_music_list_title_render}
                {
                    for music_list.iter().map(|item| {
                        let props = yew::props!(CurrentPlayListProps {
                            name: item.name.clone(),
                            is_share:item.is_share,
                            author:item.author.clone(),
                            time:item.time,
                            on_click:get_current_list_item.clone(),
                            url: item.url.clone(),
                        });
                        html!(
                            <CurrentPlayList
                                ..props.clone()
                            />
                        )
                    })
                }
            </div>
        }
    };

    let on_previous = {
        let music_list = music_list.clone();
        let current_play_index = current_selected_play_message.clone();
        let get_current_list_item = get_current_list_item.clone();
        Callback::from(move |_:MouseEvent| {
            if music_list.len() == 0 {
                return;
            }
            let new_index:i32 = if *current_play_index == 0 {
                music_list.len() as i32 - 1
            } else {
                *current_play_index - 1
            };
            current_play_index.set(new_index);
            let song = music_list[new_index as usize].clone();
            get_current_list_item.emit(song);
        })
    };

    let on_next = {
        let music_list = music_list.clone();
        let current_play_index = current_selected_play_message.clone();
        let get_current_list_item = get_current_list_item.clone();
        Callback::from(move |_| {
            if music_list.len() == 0 {
                return;
            }
            let new_index = (*current_play_index + 1) % music_list.len() as i32;
            current_play_index.set(new_index);
            let song = music_list[new_index as usize].clone();
            get_current_list_item.emit(song);
        })
    };

    let on_end_ed = {
        let on_next = on_next.clone();
        Callback::from(move |e| {
            on_next.emit(MouseEvent::new("click").unwrap());
        })
    };

    let play_main_render = {
        html! {
            <audio 
            ref={audio_ref.clone()} 
            src={play_message.url.clone()} 
            onended={on_end_ed}/>
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
                        {play_message.name.clone()}
                    </div>
                    <div class="music-player-play-author">
                        {play_message.author.clone()}
                    </div>
                </div>
            </div>
            <div class="music-player-play-wrapper">
                <div class="music-player-play-action">
                    <div class={classes!("retweet","music_player_btn")}>
                        <RetweetIcon/>
                    </div>
                    <div onclick={on_previous} class={classes!("previous","music_player_btn")}>
                        <BackwardIcon/>
                    </div>
                    <div
                        class={classes!("play_or_pause","music_player_btn")}
                        onclick={on_click_play_or_pause}
                    >
                        {play_or_pause_render}
                    </div>
                    <div onclick={on_next} class={classes!("next","music_player_btn")}>
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
