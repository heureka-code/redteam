use crate::components::{
    auth::{LoginPage, LogoutPage},
    CrackingPage, HomePage, InfoPage, NotFoundPage,
};
use crate::MainRoute;
use yew::prelude::*;

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html!(<HomePage/>),
        MainRoute::Login => html!(<LoginPage/>),
        MainRoute::Logout => html!(<LogoutPage/>),
        MainRoute::Cracking => html!(<CrackingPage/>),
        MainRoute::Info => html!(<InfoPage/>),
        MainRoute::NotFound => html!(<NotFoundPage/>),
    }
}
