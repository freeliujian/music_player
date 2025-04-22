use yew::prelude::*;
use yew_router::Switch;
use crate::header::header::Header;
use crate::router::{switch, MainRoute};

#[function_component(AppLayout)]
pub fn app_layout() -> Html {
    html! {
        <>
            <Header/>
            <Switch<MainRoute> render={switch} />
        </>
    }
}