use serde::{ Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::home_page::header::application_control_menu::styles::styles;
use crate::icons::boder_icon::BorderIcon;
use crate::icons::close_icon::CloseIcon;
use crate::icons::line_icon::LineIcon;
use crate::icons::nav_struct::Props;
use common_const::window_control_common_const::ControlWindowType;
use crate::icons::block_icon::BlockIcon;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
#[derive(Serialize)]
struct WindowArgsAction<'a> {
    args: &'a ControlWindowType,
}

struct ControlButtonParams<'a> {
    render: Html,
    label: &'a ControlWindowType,
}

#[function_component(ApplicationControlMenu)]
pub fn application_control_menu() -> Html {
    let classes = styles();
    let is_max = use_state(|| false);
    let line_and_border_and_close_props = Props {
        width:String::from("20px"),
        height:String::from("20px"),
        color:String::from("#fff"),
        class_name:String::from(" ")
    };

    fn window_action(label: ControlWindowType) {
        spawn_local(async move {
            let args = WindowArgsAction {
                args: &label
            };

            let args = serde_wasm_bindgen::to_value(&args).expect("failed to serialize window");
            invoke("control_window", args).await;
        });
    }

    let control_window = {
        Callback::from(move |label: ControlWindowType| {
           window_action(label);
        })
    };


    let control_buttons:Vec<ControlButtonParams> = vec![
        ControlButtonParams {
            render: html! {
               <LineIcon
                ..line_and_border_and_close_props.clone()
                />
            },
            label: &ControlWindowType::Hide
        },
        ControlButtonParams {
            render: html! {
                <>
                    if !*is_max.clone() {
                     <BorderIcon
                         ..line_and_border_and_close_props.clone()
                        />
                    }else {
                        <BlockIcon
                         ..line_and_border_and_close_props.clone()
                        />
                    }
                </>
            },
            label: if !*is_max.clone() { &ControlWindowType::Maximize } else { &ControlWindowType::Normal },
        },
        ControlButtonParams {
            render: html! {
               <CloseIcon
                 ..line_and_border_and_close_props.clone()
                />
            },
            label: &ControlWindowType::Close
        }
    ];

    /* language=html*/  html! {
        <div class={classes}>
            { for control_buttons.into_iter().map(|item| {
                let value = is_max.clone();
                let onclick = control_window.reform(move |_:MouseEvent| {
                    value.set(!*value);
                    item.label.clone()
                });
                html! {
                    <div onclick={onclick}>
                        {item.render}
                    </div>
                }
            }) }
        </div>
    }
}