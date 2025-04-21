mod app;
mod home_page;
mod styles;
mod router;
pub mod icons;
mod components;
pub mod header;

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
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(Level::Debug).expect("Failed to initialize logger");
         console_error_panic_hook::set_once();
    }
    yew::Renderer::<Core>::new().render();
}