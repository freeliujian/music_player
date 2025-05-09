use std::vec;

use yew::prelude::*;
use crate::found_music_page::styles::styles;
use crate::components::card::card::{Card, CardProps};
use crate::components::top_list_card::top_list_card::{TopListCard, TopListCardProps};

#[derive(Clone)]
struct  TopListProps {
    pub color: String,
    pub title: String,
    pub content: Html,
}


#[function_component(FoundMusicPage)]
pub fn found_music_page() -> Html {
    let classes = styles();
    let music_order_list = use_state(|| {
        vec![
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/X3lwU-K8ahwkswzKmm_xMA==/3435973846756513.jpg?param=140y140"), 
                text: String::from("一曲戏腔极尽风流，一抹笑靥极尽欢颜。"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/ObgkcuElyoZwxppjuPOYOw==/109951169170452068.jpg?param=140y140"), 
                text: String::from("纯音乐」你偶尔需要安静的发泄"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/DTA1vQiLtNxZZe7dDWI6Qg==/109951164187665128.jpg?param=140y140"), 
                text: String::from("节奏控 | 那些超带感的英文歌"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
        ]
    });

    let good_recommeded_list = use_state(|| 
        vec![
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/X3lwU-K8ahwkswzKmm_xMA==/3435973846756513.jpg?param=140y140"), 
                text: String::from("一曲戏腔极尽风流，一抹笑靥极尽欢颜。"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/ObgkcuElyoZwxppjuPOYOw==/109951169170452068.jpg?param=140y140"), 
                text: String::from("纯音乐」你偶尔需要安静的发泄"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/DTA1vQiLtNxZZe7dDWI6Qg==/109951164187665128.jpg?param=140y140"), 
                text: String::from("节奏控 | 那些超带感的英文歌"),
                extra_text: String::from("根据你的喜好推荐歌曲"),
            },
        ]
    );

        let top_list_props = use_state(|| {
        vec![
            TopListProps {
                title: String::from("热歌榜"),
                color: String::from("#d77a94"),
                content: {
                    html!(
                        <div>
                            <p>
                                {"1.格桑花"}
                            </p>
                        </div>
                    )
                }
            },
            TopListProps {
                title: String::from("热歌榜"),
                color: String::from("#d77a94"),
                content: {
                    html!(
                        <div>
                            <p>
                                {"1.格桑花"}
                            </p>
                        </div>
                    )
                }
            },
            TopListProps {
                title: String::from("热歌榜"),
                color: String::from("#d77a94"),
                content: {
                    html!(
                        <div>
                            <p>
                                {"1.格桑花"}
                            </p>
                        </div>
                    )
                }
            },
            TopListProps {
                title: String::from("热歌榜"),
                color: String::from("#d77a94"),
                content: {
                    html!(
                        <div>
                            <p>
                                {"1.格桑花"}
                            </p>
                        </div>
                    )
                }
            },
        ]
    });


    let recommended_playlist_card_render = {
        let music_order_list = music_order_list.clone();
        html!{
            <>
               {
                    for <Vec<CardProps> as Clone>::clone(&music_order_list).into_iter().map(|item| {
                        html!(
                            <>
                                <Card
                                    img_url={item.img_url.clone()}
                                    url={item.url.clone()}
                                    text={item.text.clone()}
                                    extra_text={item.extra_text.clone()}
                                />
                            </>
                        )
                    })
               }
            </>
        }
    };

    let good_recommended_list_car_render = {
        let good_recommeded_list = good_recommeded_list.clone();
        html!(
            <>
                {
                    for <Vec<CardProps> as Clone>::clone(&good_recommeded_list).into_iter().map(|item| {
                        html!(
                            <>
                                <Card
                                    img_url={item.img_url.clone()}
                                    url={item.url.clone()}
                                    text={item.text.clone()}
                                    extra_text={item.extra_text.clone()}
                                />
                            </>
                        )
                    })
                }
            </>
        )
    };


    let top_list_render = {
        let top_list_props = top_list_props.clone();
        html!(
            <ul class={"top_list_wrapper"}>
                {
                    for <Vec<TopListProps> as Clone>::clone(&top_list_props).into_iter().map(|item| {
                        let on_click = Callback::from(move |value: String| {
                            log::info!("clicked top list {}", value);
                        });
                        html!(
                            <TopListCard 
                                title={item.title.clone()}
                                color={item.color.clone()}
                                content= {
                                item.content.clone()
                                }
                                on_click={on_click}
                            />
                        )
                    })
                }
             
            </ul>
        )
    };

    html! {
        <div class={classes}>
            <div class="title">
                {"发现音乐"}
            </div>
            <div class="content">
                <div class="carousel">

                </div>
                <div class="title">
                    {"推荐歌单 >"}
                </div>
                <div class="song-list">
                   {recommended_playlist_card_render}
                </div>
                <div class="title">
                    {"精彩推荐 >"}
                </div>
                <div class="song-list">
                    {good_recommended_list_car_render}
                </div>
                <div class="title">
                    {"排行榜 >"}
                </div>
                <div class="song-list">
                    {top_list_render}
                </div>
            </div>
        </div>
    }
} 
