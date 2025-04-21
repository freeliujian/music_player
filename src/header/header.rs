use yew::prelude::*;
use yew::props;
use crate::header::header_style::styles;
use yew_router::hooks::use_navigator;
use crate::icons::right_icon::{ RightIcon};
use crate::icons::left_icon::{ LeftIcon};
use crate::router::Route::Home;
use crate::icons::nav_struct::Props;

use crate::header::right_config::right_config::RightConfig;

#[function_component(Header)]
pub fn header_component() -> Html {
    let header_style = styles();
    let router = use_navigator().expect("Navigator not found");
    let handle_click =move |_| {
        router.push(&Home)
    };

    let nav_icons_props =Some(props!(Props {
         width: "16px",
            height:"16px",
            color: "#ffffff",
    }));

    /* language=html*/  html! {
        <div class={header_style}>
            <div class="header-left">
                <div onclick={handle_click} class="log_img" >
                    <img src="public/yew.png"  alt="logo"/>
                    <span>{"yyds"}</span>
                </div>
            </div>

            <div class="header-center ">
                  <div class="nav-btn">
                    <LeftIcon
                        ..nav_icons_props.clone().unwrap()
                    />
                </div>
                <div class="nav-btn">
                    <RightIcon
                        ..nav_icons_props.clone().unwrap()
                    />
                </div>
                <div class="search-box">
                    {111}
                </div>
            </div>
            <RightConfig/>

        </div>
    }
}