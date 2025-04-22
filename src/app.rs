use yew::prelude::*;
use crate::router::{switch, MainRoute};
use yew_router::prelude::*;
use crate::header::header::Header;
use crate::config_provide::context::{ThemeProvider};

#[function_component(AppLayout)]
fn app_layout() -> Html {
    html! {
        <>
            <Header/>
            <Switch<MainRoute> render={switch} />
        </>
    }
}

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

