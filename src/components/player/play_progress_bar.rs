use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use web_sys::{MouseEvent};
use gloo::utils::{document, window};
use crate::components::player::styles::styles;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

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

    let on_mouse_move = {
        let is_dragging = is_dragging.clone();
        let progress_container = progress_container.clone();
        let duration = *duration;
        let on_seek = on_seek.clone();

        Callback::from(move |e: MouseEvent| {
            // 是否开启拖动
            if *is_dragging {
                if let Some(container) = progress_container.cast::<web_sys::Element>() {
                    let rect = container.get_bounding_client_rect();
                    let x = e.client_x() as f64 - rect.left();
                    let width = rect.width();
                    if width > 0.0 {
                        let percent = (x / width).clamp(0.0, 1.0);
                        let new_time = (percent * duration as f64) as u64;
                        on_seek.emit(new_time);
                    }
                }
            }
        })
    };

    let on_mouse_up = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_:MouseEvent| {
            is_dragging.set(false);
        })
    };

    use_effect_with(
        (*is_dragging, on_mouse_move.clone(), on_mouse_up.clone()),
        move |(is_dragging, on_mouse_move, on_mouse_up)| {
            // 创建一个多重所有权得闭包
            let closures = Rc::new(RefCell::new(Vec::<Closure<dyn FnMut(MouseEvent)>>::new()));
            let blur_closures = Rc::new(RefCell::new(Vec::<Closure<dyn FnMut()>>::new()));

            let move_emit = on_mouse_move.clone();
            let up_emit = on_mouse_up.clone();
            let blur_emit = on_mouse_up.clone();
            if *is_dragging {

                log::info!("on_dragging");

                let move_closure = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                    log::info!("on_mouse_move");
                    move_emit.emit(e);
                });
                let up_closure = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                    log::info!("on_mouse_up");
                    up_emit.emit(e);
                });

                let blur_closure = Closure::<dyn FnMut()>::new(move || {
                    if let Ok(e) = MouseEvent::new("blur") {
                        log::info!("blur");
                        blur_emit.emit(e);
                    }
                });

                document()
                    .add_event_listener_with_callback("mousemove", move_closure.as_ref().unchecked_ref())
                    .expect("Failed to add mousemove listener");
                document()
                    .add_event_listener_with_callback("mouseup", up_closure.as_ref().unchecked_ref())
                    .expect("Failed to add mouseup listener");
                window()
                    .add_event_listener_with_callback("blur", blur_closure.as_ref().unchecked_ref())
                    .expect("Failed to add blur listener");

                closures.borrow_mut().push(move_closure);
                closures.borrow_mut().push(up_closure);
                blur_closures.borrow_mut().push(blur_closure);

            }

            move || {
                let closures = closures.borrow();
                let blur_closures = blur_closures.borrow();

                for closure in closures.iter() {
                    document()
                        .remove_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                        .ok();
                    document()
                        .remove_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
                        .ok();
                }

                for closure in blur_closures.iter() {
                    window()
                        .remove_event_listener_with_callback("blur", closure.as_ref().unchecked_ref())
                        .ok();
                }
            }
        },
    );
    
    let on_click = {
        let progress_container = progress_container.clone();
        let duration = *duration;
        let on_seek = on_seek.clone();

        Callback::from(move |e: MouseEvent| {
            if let Some(container) = progress_container.cast::<web_sys::Element>() {
                let rect = container.get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let width = rect.width();
                if width > 0.0 {
                    let percent = (x / width).clamp(0.0, 1.0);
                    let new_time = (percent * duration as f64) as u64;
                    on_seek.emit(new_time);
                }
            }
        })
    };


    html! {
        <div class={classes}>
            <span class="time-display">{ format_time(*current_time) }</span>

            <div
                class="progress-bar-wrapper"
                ref={progress_container}
                onclick={on_click}
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