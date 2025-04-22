use yew::prelude::*;
use crate::header::search::styles::styles;
use crate::components::input::input_base::{ InputBase, InputBaseProps };
use std::rc::Rc;
use crate::icons::nav_struct::Props;
use crate::icons::search_icon::SearchIcon;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchInputProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_search: Callback<String>,
    #[prop_or_default]
    pub on_change: Callback<String>,
}

#[function_component(SearchInput)]
pub fn search_input(props: &SearchInputProps) -> Html {
    let styles = styles();
    let value = Rc::new(props.value.clone());

    let value_for_click = Rc::clone(&value);
    let value_for_keypress = Rc::clone(&value);


    let memoized_callback = use_memo(
        props.on_search.clone(),
        |on_search| {
            on_search.clone()
        },
    );
    let memoized_on_click = (*memoized_callback).clone();
    let memoized_on_keypress = (*memoized_callback).clone();

    let on_change = props.on_change.clone();

    // let on_click = Callback::from(move |_| {
    //     memoized_on_click.emit((*value_for_click).clone());
    // });

    let on_key_press = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            memoized_on_keypress.emit((*value_for_keypress).clone());
        }
    });

    let search_icon_props = Props {
        width:String::from("20px"),
        height:String::from("20px"),
        color:String::from("#fff"),
        class_name:String::from(" ")
    };
  
    let render_prefix = html!(
        <SearchIcon ..search_icon_props.clone()/>
    );

    html! {
        <div class={classes!("search-input", styles)}>
            <InputBase
                value={(*value).clone()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                on_change={on_change}
                prefix ={render_prefix}
            />
        </div>
    }
} 
