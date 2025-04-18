mod app;
mod home_page;
mod styles;

use yew_router::prelude::*;
use yew::prelude::*;
use app::App;
use home_page::home::HomePage;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/template")]
    Template,
}

fn switch(routes: Route) -> Html {
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
    }
}

#[function_component(Core)]
fn core() -> Html {
    html! {
     <BrowserRouter>
        <Switch<Route> render={switch} />
     </BrowserRouter>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Core>::new().render();
}