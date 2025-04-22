use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::app_layout::app_layout::AppLayout;
use crate::config_provide::context::{ThemeProvider};


#[function_component(App)]
pub fn app() -> Html {
   html! {
       <>
        <ThemeProvider>
         <BrowserRouter>
            <AppLayout/>
         </BrowserRouter>
       </ThemeProvider>
        </>
   }
}

