use yew::prelude::*;
use yew::props;
use yew_router::hooks::use_navigator;
use crate::header::right_config::application_control_menu::application_control_menu::ApplicationControlMenu;
use crate::header::right_config::styles::styles;
use crate::icons::mail::MailIcon;
use crate::router::Route::Settings;
use crate::icons::user_icon::{UserIcon, UserIconProps};
use crate::icons::nav_struct::Props;
use crate::icons::setting_icon::SettingIcon;
use crate::icons::wrench_icon::SetThemeIcon;

#[function_component(RightConfig)]
pub fn right_config() -> Html {
    let classes = styles();
    let router = use_navigator().expect("Navigator not found");
    let handle_click =move |_| {
        router.push(&Settings)
    };

    let user_icons_props = props!(UserIconProps {
            width: "20px",
            height:"20px",
            color: "#ffffff",
    });
    let set_and_theme_and_mail_icons_props =Some(props!(Props {
         width: "20px",
            height:"20px",
            color: "#ffffff",
    }));
    let un_login = "未登录";

    /* language=html*/  html! {
        <div class={classes}>
            <div class="user-avatar">
                <UserIcon
                    ..user_icons_props
                />
            </div>
            <div class="user-status">
                {un_login}
            </div>
            <div class="config_wrapper">
                 <div class="header-theme">
                <SetThemeIcon
                    ..set_and_theme_and_mail_icons_props.clone().unwrap()
                />
                </div>
                <div
                    onclick={handle_click}
                    class="setting">
                    <SettingIcon
                        ..set_and_theme_and_mail_icons_props.clone().unwrap()
                    />
                </div>
                <div class="mail">
                    <MailIcon
                        ..set_and_theme_and_mail_icons_props.clone().unwrap()
                    />
                </div>
            </div>
            <div class="divider"/>
            <ApplicationControlMenu />
        </div>
    }
}