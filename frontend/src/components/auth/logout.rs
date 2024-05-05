use crate::{stores::AuthStore, MainRoute};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(LogoutPage)]
pub fn logout() -> Html {
    let (_state, dispatch) = use_store::<AuthStore>();
    let history = use_navigator().unwrap();

    dispatch.set(AuthStore::default());
    history.push(&MainRoute::Login);
    html!()
}
