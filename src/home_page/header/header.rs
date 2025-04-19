use yew::prelude::*;
use stylist::yew::styled_component;
use crate::home_page::header::header_style::styles;
use yew_router::hooks::use_navigator;
use crate::router::Route::Home;


#[styled_component(Header)]
pub fn header_component() -> Html {
    let header_style = styles();
    let router = use_navigator().expect("Navigator not found");
    let handle_click =move |_| {
        router.push(&Home)
    };



    /* language=html */   html! {
        <div class={header_style}>
            <div class="header-left">
                <div onclick={handle_click} class="log_img" >
                    <img src="public/yew.png"  alt="logo"/>
                    <span>{"yyds"}</span>
                </div>
            </div>
        </div>
    }
}