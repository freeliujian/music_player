use yew::prelude::*;
use yew::context::ContextProvider;
use std::rc::Rc;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Theme {
    #[prop_or_default]
    pub color_primary: String,
    #[prop_or_default]
    pub color_text_base: String,
    #[prop_or_default]
    pub color_bg_base: String,
    #[prop_or_default]
    pub color_success: String,
    #[prop_or_default]
    pub color_error: String,
    #[prop_or_default]
    pub color_warning: String,
    #[prop_or_default]
    pub color_info: String,
    #[prop_or_default]
    pub border_radius: String,
    #[prop_or_default]
    pub space: String,
    #[prop_or_default]
    pub font_family: String,
    #[prop_or_default]
    pub font_size: String,
}
#[derive(Properties, PartialEq, Clone)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Html,
}

pub type ThemeContextProvider = Rc<Theme>;

#[function_component(ThemeProvider)]
pub fn theme(props: &ThemeProviderProps) -> Html {
    let theme_context = Rc::new(Theme {
        color_primary: String::from(" #1677ff"),
        color_text_base: String::from("#000"),
        color_bg_base: String::from(" #ffffff"),
        color_warning: String::from(" #faad14"),
        color_success: String::from(" #52c41a"),
        color_info: String::from(" #1677ff"),
        color_error: String::from(" #ff4d4f"),
        border_radius: String::from("6px"),
        space:String::from("8px"),
        font_family: String::from("-apple-system, BlinkMacSystemFont,sans-serif"),
        font_size: String::from("14px"),
    });
    html!(
        <ContextProvider<ThemeContextProvider> context={theme_context}>
            {props.children.clone()}
        </ContextProvider<ThemeContextProvider> >
    )
}