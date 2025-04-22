use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::App;
use crate::home_page::home::HomePage;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/home/foundMusic")]
    HomeRedirect,
    #[at("/login")]
    Login,
    #[at("/template")]
    Template,
    #[at("/settings")]
    Settings,
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum subRouteForHomeRoute {
    #[at("/home/foundMusic")]
    FoundMusic,
    #[at("/home/creatorOrPlayer")]
    CreatorOrPlayer,
    #[at("/home/video")]
    HomeVideo,
    #[at("/home/focus")]    
    Focus,
    #[at("/home/stream")]
    Stream,
    #[at("/home/selfData")]
    SelfData
}

pub fn home_switch(routes: subRouteForHomeRoute) -> Html {
    match routes {
        subRouteForHomeRoute::FoundMusic => html! {
            <h1>{"found music"}</h1>
        },
        subRouteForHomeRoute::CreatorOrPlayer => html! {
            <h1>{"creator or player"}</h1>
        },
        subRouteForHomeRoute::HomeVideo => html! {
            <h1>{"home video"}</h1>
        },
        subRouteForHomeRoute::Focus => html! {
            <h1>{"focus"}</h1>
        },
        subRouteForHomeRoute::Stream => html! {
            <h1>{"stream"}</h1>
        },
        subRouteForHomeRoute::SelfData => html! {
            <h1>{"self"}</h1>
        }
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage/>
        },
        Route::HomeRedirect => html! {
            <Redirect<Route> to={Route::Home}/>
        }
        Route::Login => html! {
            <h1>{"Login"}</h1>
        },
        Route::Template => html! {
            <App/>
        },
        Route::Settings => html! {
            <h1>{"Settings"}</h1>
        }
    }
}
