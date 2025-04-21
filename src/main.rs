mod app;
mod home_page;
mod styles;
mod router;
pub mod icons;
mod components;
pub mod header;

use yew::prelude::*;
use log::Level;
use app::App;





fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(Level::Debug).expect("Failed to initialize logger");
         console_error_panic_hook::set_once();
    }
    yew::Renderer::<App>::new().render();
}