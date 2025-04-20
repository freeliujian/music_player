use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::home_page::header::application_control_menu::styles::styles;
use crate::icons::boder_icon::BorderIcon;
use crate::icons::close_icon::CloseIcon;
use crate::icons::line_icon::LineIcon;
use crate::icons::nav_struct::Props;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(ApplicationControlMenu)]
pub fn application_control_menu() -> Html {
    let classes = styles();

    let line_and_border_and_close_props = Props{
        width:String::from("20px"),
        height:String::from("20px"),
        color:String::from("#fff"),
        class_name:String::from(" ")
    };

    let close_window = move |_| {
        spawn_local(async {
            let args = serde_wasm_bindgen::to_value(&()).unwrap();
            invoke("close_window", args).await;
        })
    };


    /* language=html*/  html! {
        <div class={classes}>
            <div class="min">
                <LineIcon
            ..line_and_border_and_close_props.clone()
            />
            </div>
            <div class="max">
                <BorderIcon
             ..line_and_border_and_close_props.clone()
            />
            </div>
            <div class="close" onclick={close_window}>
                <CloseIcon
             ..line_and_border_and_close_props.clone()
            />
            </div>
        </div>
    }
}