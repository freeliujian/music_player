mod app;
mod home_page;
mod styles;
pub mod router;
pub mod icons;
pub mod components;
pub mod header;
mod config_provide;

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