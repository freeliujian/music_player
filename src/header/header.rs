use yew::prelude::*;
use yew::props;
use crate::header::header_style::styles;
use yew_router::hooks::{use_navigator, use_route};
use crate::icons::right_icon::{ RightIcon };
use crate::icons::left_icon::{ LeftIcon };
use crate::router::MainRoute::HomeRoot;
use crate::router::SubForHomeRoute::FoundMusic;
use crate::icons::nav_struct::Props;
use crate::header::right_config::right_config::RightConfig;
use crate::header::search::search::{SearchInput, SearchInputProps};
use crate::config_provide::context::ThemeContextProvider;
use crate::router::MainRoute;

#[function_component(Header)]
pub fn header_component() -> Html {
    let theme_context = use_context::<ThemeContextProvider>().expect("not get Theme context");
    let header_style = styles(&*theme_context);
    let location = use_navigator().expect("Navigator not found");
    let effect_location = location.clone();
    let router = use_route::<MainRoute>().expect("Main route not found");
    let handle_click =move |_| {
        location.push(&FoundMusic);
    };

    let nav_icons_props =Some(props!(Props {
         width: "16px",
            height:"16px",
            color: "#ffffff",
    }));

    let search_bar_props = Some(props!(SearchInputProps {
        value: "".to_string(),
    }));

    use_effect_with (
        (),
        move |_| {
            if router == HomeRoot {
                effect_location.push(&FoundMusic);
            }
        }
    );

    html! {
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
                    <SearchInput
                        ..search_bar_props.unwrap() 
                    />
                </div>
            </div>
            <RightConfig/>
        </div>
    }
}