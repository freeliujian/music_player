use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::prelude::*;
use crate::home_page::home_style::styles;
use crate::home_page::sidebar::sidebar::Siderbar;
use crate::router::{home_switch, SubForHomeRoute};

#[styled_component(HomePage)]
pub fn home_page() -> Html {
    let home_style = styles();
    /* language=html */   html! {
        <mian class={home_style}>
            <div class="app-container">
                <Siderbar/>
                <div class="main-content">
                    <Switch<SubForHomeRoute> render={home_switch} />
                </div>
                <div class="player-bar">
                    {"播放器"}
                </div>
            </div>
        </mian>
    }
}