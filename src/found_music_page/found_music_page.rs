use yew::prelude::*;
use crate::found_music_page::styles::styles;
use crate::components::card::card::{Card, CardProps};


#[function_component(FoundMusicPage)]
pub fn found_music_page() -> Html {
    let classes = styles();
    let music_order_list = use_state(|| {
        vec![
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人")
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/X3lwU-K8ahwkswzKmm_xMA==/3435973846756513.jpg?param=140y140"), 
                text: String::from("一曲戏腔极尽风流，一抹笑靥极尽欢颜。")
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/ObgkcuElyoZwxppjuPOYOw==/109951169170452068.jpg?param=140y140"), 
                text: String::from("纯音乐」你偶尔需要安静的发泄")
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/DTA1vQiLtNxZZe7dDWI6Qg==/109951164187665128.jpg?param=140y140"), 
                text: String::from("节奏控 | 那些超带感的英文歌")
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人")
            },
            CardProps {
                url: String::from("/"), 
                img_url: String::from("http://p1.music.126.net/eOTnvmGVSb3S2MIYmo2IGA==/109951163910753373.jpg?param=140y140"), 
                text: String::from("你曾经有多喜欢一个人")
            },
        ]
    });

    let card_render = {
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
                                />
                            </>
                        )
                    })
               }
            </>
        }
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
                    {"推荐歌单"}
                </div>
                <div class="song-list">
                   {card_render}
                </div>
            </div>
        </div>
    }
} 
