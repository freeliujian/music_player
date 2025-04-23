use yew::prelude::*;
use web_sys::{MouseEvent,HtmlElement};
use gloo::utils::document;
use crate::components::player::styles::styles;
use wasm_bindgen::closure::Closure;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub current_time: u64, 
    pub duration: u64,     
    pub on_seek: Callback<u64>, 
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let ProgressBarProps {
        current_time,
        duration,
        on_seek,
    } = props;
    let classes = styles();
    // 是否正在拖动
    let is_dragging = use_state(|| false);
    // 进度条容器的引用
    let progress_container = use_node_ref();

    // 格式化时间为 MM:SS
    fn format_time(seconds: u64) -> String {
        let minutes = seconds / 60;
        let seconds = seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    // 计算进度百分比
    let progress_percent = if *duration > 0 {
        (*current_time as f64 / *duration as f64) * 100.0
    } else {
        0.0
    };

    // 处理鼠标按下事件（开始拖动）
    let on_mouse_down = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            is_dragging.set(true);
        })
    };


    html! {
        <div class={classes}>
            <span class="time-display">{ format_time(*current_time) }</span>
            
            <div 
                class="progress-bar-wrapper"
                ref={progress_container}
                // onclick={on_click}
            >
                <div class="progress-bar-bg">
                    <div 
                        class="progress-bar-progress" 
                        style={format!("width: {}%", progress_percent)}
                    >
                        <div 
                            class="progress-bar-thumb"
                            onmousedown={on_mouse_down}
                        ></div>
                    </div>
                </div>
            </div>
            
            <span class="time-display">{ format_time(*duration) }</span>
        </div>
    }
}