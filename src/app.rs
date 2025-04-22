use yew::prelude::*;
use crate::router::{switch, Route};
use yew_router::prelude::*;
use crate::header::header::Header;
use crate::config_provide::context::{ThemeProvider};


#[function_component(App)]
pub fn app() -> Html {
   html! {
       <>
        <ThemeProvider>
         <BrowserRouter>
            <Header/>
            <Switch<Route> render={switch} />
         </BrowserRouter>
       </ThemeProvider>
        </>
   }
}
