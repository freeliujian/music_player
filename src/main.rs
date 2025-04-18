mod app;
mod home_page;
mod styles;
mod router;

use yew_router::prelude::*;
use yew::prelude::*;
use crate::router::{switch, Route};

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