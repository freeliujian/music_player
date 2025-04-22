use yew::prelude::*;
use crate::components::input::styles::styles;

#[derive(Properties, PartialEq, Clone)]
pub struct InputBaseProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub size: InputSize,
    #[prop_or_default]
    pub prefix: Option<Html>,
    #[prop_or_default]
    pub suffix: Option<Html>,
    #[prop_or_default]
    pub on_change: Callback<String>,  
}

#[derive(PartialEq, Clone)]
pub enum InputSize {
    Small,
    Middle,
    Large,
}

impl Default for InputSize {
    fn default() -> Self {
        InputSize::Middle
    }
}

#[function_component(InputBase)]
pub fn input_base(props: &InputBaseProps) -> Html {
    let styles = styles();
    let value = props.value.clone();
    let on_change = props.on_change.clone();
    
    let oninput = Callback::from(move |e: InputEvent| {
        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
        on_change.emit(input.value());
    });

    let input_class = classes!(
        "input",
        match props.size {
            InputSize::Small => "input-sm",
            InputSize::Large => "input-lg",
            _ => "",
        },
        if props.disabled { "input-disabled" } else { "" }
    );

    html! {
        <div class={classes!( styles)}>
            if let Some(prefix) = &props.prefix {
                <span class="input-prefix">{prefix.clone()}</span>
            }
            <input
                class={input_class}
                type="text"
                value={value}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                oninput={oninput}
            />
            if let Some(suffix) = &props.suffix {
                <span class="input-suffix">{suffix.clone()}</span>
            }
        </div>
    }
}
