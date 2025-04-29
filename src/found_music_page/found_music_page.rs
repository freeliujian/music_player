use yew::prelude::*;
use crate::found_music_page::styles::styles;
use crate::components::card::card::Card;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchInputProps {

}

#[function_component(FoundMusicPage)]
pub fn found_music_page() -> Html {
    let classes = styles();
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
                    <Card/>
                </div>
            </div>
        </div>
    }
} 
