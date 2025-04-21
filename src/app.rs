use yew::prelude::*;
use crate::router::{switch, Route};
use yew_router::prelude::*;
use crate::header::header::Header;


#[function_component(App)]
pub fn app() -> Html {
   html! {
       <>
         <BrowserRouter>
            <Header/>
            <Switch<Route> render={switch} />
         </BrowserRouter>
        </>
   }
}
