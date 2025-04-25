use yew::prelude::*;
use yew_router::prelude::*;
// use crate::app::App;
use crate::home_page::home::HomePage;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum MainRoute {
    #[at("/")]
    HomeRoot,
    #[at("/home/*")]
    Home,
    #[at("/login")]
    Login,
    #[at("/template")]
    Template,
    #[at("/settings")]
    Settings,
    
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum SubForHomeRoute {
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

pub fn home_switch(routes: SubForHomeRoute) -> Html {
    match routes {
        SubForHomeRoute::FoundMusic => html! {
            <Redirect<MainRoute> to={MainRoute::HomeRoot}/>
        },
        SubForHomeRoute::CreatorOrPlayer => html! {
            <h1>{"creator or player"}</h1>
        },
        SubForHomeRoute::HomeVideo => html! {
            <h1>{"home video"}</h1>
        },
        SubForHomeRoute::Focus => html! {
            <h1>{"focus"}</h1>
        },
        SubForHomeRoute::Stream => html! {
            <h1>{"stream"}</h1>
        },
        SubForHomeRoute::SelfData => html! {
            <h1>{"self"}</h1>
        }
    }
}

pub fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::HomeRoot 
        | MainRoute::Home  
        => html! {
            <HomePage/>
        },
        MainRoute::Login => html! {
            <h1>{"Login"}</h1>
        },
        MainRoute::Template => html! {
            <h1>{"Template"}</h1>
        },
        MainRoute::Settings => html! {
            <h1>{"Settings"}</h1>
        }
    }
}
