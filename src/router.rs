use yew::{html, Html};
use yew_router::Routable;
use crate::app::App;
use crate::home_page::home::HomePage;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/template")]
    Template,
    #[at("/settings")]
    Settings,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage/>
        },
        Route::Login => html! {
            <h1>{"Login"}</h1>
        },
        Route::Template => html! {
            <App/>
        },
        Route::Settings => html! {
            <h1>{"Settings"}</h1>
        }
    }
}
