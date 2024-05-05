use crate::stores::AuthStore;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::auth::protect_function_with_login;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    use crate::components::{Page, Pettable, TextInput};

    let (_state, dispatch) = use_store::<AuthStore>();
    let history = use_navigator().unwrap();

    let state = use_state(|| None::<String>);

    let cloned_state = state.clone();
    let pattern_changed = move |pattern: String| {
        cloned_state.set(pattern.into());
    };

    protect_function_with_login!(dispatch, history);

    let username = dispatch.get().username.clone().unwrap_or("".to_string());
    let name_pattern = state.as_ref().map(|s| s.to_owned()).unwrap_or_default();
    html!(
        <Page title={"Hello ".to_owned() + &username} logout=true key_icon=true>
            <TextInput class={classes!("pet-name-pattern-edit")} onchange={pattern_changed} name={"pattern"} placeholder={"Type the pattern to search for here, '_' replaces any single letter, '%' any amount of letters. Use enter for update..."}/>
            <Pettable name_pattern={name_pattern}/>
        </Page>
    )
}
