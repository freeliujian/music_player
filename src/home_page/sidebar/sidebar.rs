use yew::prelude::*;
use crate::config_provide::context::{Theme, ThemeContextProvider};
use crate::home_page::sidebar::styles::styles;
use yew_router::prelude::*;
use crate::router::Route;

pub struct MenuListProps {
    pub label: String,
    pub value: String,
}

#[function_component(Siderbar)]
pub fn sider_bar() -> Html {
    let theme_context = use_context::<ThemeContextProvider>().expect("Theme not set");
    let current_selected = use_state(|| String::from("发现音乐"));
    let location = use_navigator().unwrap();
    let menu_list_main = vec![
        MenuListProps{
            label: String::from("发现音乐"),
            value: String::from("foundMusic"),
        },
        MenuListProps{
            label: String::from("播客"),
            value: String::from("creatorOrPlayer"),
        },
        MenuListProps{
            label: String::from("视频"),
            value: String::from("video"),
        },
        MenuListProps{
            label: String::from("关注"),
            value: String::from("focus"),
        },
        MenuListProps{
            label: String::from("直播"),
            value: String::from("stream"),
        },
        MenuListProps{
            label: String::from("私人漫游"),
            value: String::from("selfData"),
        },
    ];
    let style = styles(&*theme_context,&(current_selected.clone()));

    let on_change = {
        Callback::from(move |value: String| {
            log::info!("{}", value);
            // location.push("")
        })
    };

    html!(
        <div class={classes!(style)}>
            <div class="menu">
                {
                    for menu_list_main.into_iter().map(|menu| {
                        let on_change_current_selected = current_selected.clone();
                        let is_clicked_menu_value = menu.label.clone();
                        let get_current_selected = menu.label.clone();
                        let on_change = on_change.reform(move |_:MouseEvent| {
                            on_change_current_selected.set(is_clicked_menu_value.clone());
                            menu.value.clone()
                        });
                        let is_clicked_current_selected = current_selected.clone();
                    html!(
                        <div
                        class={
                            classes!("menu-list",if *is_clicked_current_selected == get_current_selected {"active"} else {""} )
                        }
                        onclick={on_change}
                        >
                            {menu.label.clone()}
                        </div>
                        )
                    })
                }

            </div>
        </div>
    )
}