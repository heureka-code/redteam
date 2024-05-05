use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/logout")]
    Logout,

    #[at("/crack")]
    Cracking,

    #[at("/info")]
    Info,

    #[not_found]
    #[at("/404")]
    NotFound,
}
