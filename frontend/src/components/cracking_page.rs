use crate::{api, stores::AuthStore};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::auth::protect_function_with_login;

#[function_component(CrackingPage)]
pub fn cracking_page() -> Html {
    use crate::components::{LoadingSpinner, Page, TextInput};

    let pending_state = use_state(|| false);
    let u_state = use_state(|| None::<String>);
    let h_state = use_state(|| None::<String>);
    let cracked_state = use_state(|| None::<String>);

    let cloned_u_state = u_state.clone();
    let username_changed = move |username: String| {
        cloned_u_state.set(username.into());
    };
    let cloned_h_state = h_state.clone();
    let hash_changed = move |hash: String| {
        cloned_h_state.set(hash.into());
    };

    let (_state, dispatch) = use_store::<AuthStore>();
    let history = use_navigator().unwrap();
    protect_function_with_login!(dispatch, history);

    let cloned_u_state = u_state.clone();
    let cloned_h_state = h_state.clone();
    let cloned_cracked_state = cracked_state.clone();
    let cloned_pending_state = pending_state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let cloned_cracked_state = cloned_cracked_state.clone();
        let cloned_pending_state = cloned_pending_state.clone();
        let username: Option<String> = cloned_u_state.as_ref().cloned();
        let hash: Option<String> = cloned_h_state.as_ref().cloned();

        if let (Some(username), Some(hash)) = (username, hash) {
            wasm_bindgen_futures::spawn_local(async move {
                cloned_pending_state.set(true);
                cloned_cracked_state.set(None);
                let resp = api::api_crack_password(username, hash).await;

                cloned_cracked_state.set(resp.and_then(|r| r.password().map(|s| s.to_string())));
                cloned_pending_state.set(false);
            });
        }
    });

    let cracked: &Option<String> = &cracked_state;
    let disabled = *pending_state;

    let username = dispatch.get().username.clone().unwrap_or("".to_string());
    html!(
        <Page title={"Enter your guess for the admin, ".to_owned() + &username} logout=true>
        <form onsubmit={onsubmit}>
            <TextInput class={classes!("username-guess")} onchange={username_changed} name={"username"} placeholder={"admin username"}/>
            <TextInput class={classes!("hash-guess")} onchange={hash_changed} name={"password-hash"} placeholder={"password hash"}/>
            <input name={"submitBtn"} type={"submit"} disabled={disabled}/>
        </form>
        {
            if *pending_state {
                html!(<div class={classes!("pending-label")}><LoadingSpinner/></div>)
            } else {html!()}
        }
        {
            if let Some(cracked) = cracked {
                html!(<div class={classes!("cracked-password")}><span class={classes!("cracked-password-label")}>{"Cracked: "}</span><span class={classes!("cracked-password-value")}>{cracked}</span></div>)
            } else {html!()}
        }

        </Page>
    )
}
