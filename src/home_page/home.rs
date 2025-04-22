use yew::prelude::*;
use stylist::yew::styled_component;
use crate::home_page::home_style::styles;
use crate::home_page::sidebar::sidebar::Siderbar;

#[styled_component(HomePage)]
pub fn home_page() -> Html {
    let home_style = styles();
    /* language=html */   html! {
        <mian class={home_style}>
            <div class="app-container">
                <Siderbar/>
                <div class="main-content">
                    {"首页主题布局"}
                </div>
                <div class="player-bar">
                    {"播放器"}
                </div>
            </div>
        </mian>
    }
}