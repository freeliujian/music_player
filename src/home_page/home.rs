use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::prelude::*;
use crate::home_page::home_style::styles;
use crate::home_page::sidebar::sidebar::Siderbar;
use crate::router::{home_switch, SubForHomeRoute};
use crate::components::music_player_component::music_player_component::MusicPlayerComponent;

#[function_component(SubPage)]
fn sub_other_page() -> Html {
    html! {
        <Switch<SubForHomeRoute> render={home_switch} />
    }
}

#[function_component(SecondPageRoot)]
pub fn second_page_root() -> Html {
    html! {
        {"root"}
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
                    <SubPage/>
                </div>
                <div class="player-bar">
                    <MusicPlayerComponent/>
                </div>
            </div>
        </mian>
    }
}