use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::prelude::*;
use crate::home_page::home_style::styles;
use crate::home_page::sidebar::sidebar::Siderbar;
use crate::router::{home_switch, SubForHomeRoute};
use crate::components::music_player_component::music_player_component::MusicPlayerComponent;

#[function_component(SubOtherPage)]
pub fn sub_other_page() -> Html {
    html! {
        <Switch<SubForHomeRoute> render={home_switch} />
    }
}

#[function_component(FoundMusicPage)]
pub fn found_music_page() -> Html {
    html! {
        {"111"}
    }
}

#[styled_component(HomePage)]
pub fn home_page() -> Html {
    let home_style = styles();
    html! {
        <mian class={home_style}>
            <div class="app-container">
                <Siderbar/>
                <div class="main-content">
                    <FoundMusicPage/>
                    <SubOtherPage/>
                </div>
                <div class="player-bar">
                    <MusicPlayerComponent/>
                </div>
            </div>
        </mian>
    }
}