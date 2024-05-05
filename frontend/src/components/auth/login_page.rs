use super::only_if_not_logged_in;
use crate::{api, components::Page, stores::AuthStore, MainRoute};
use common::ResponseLoginUser;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    use super::{login_form, LoginForm};

    let (_state, dispatch) = use_store::<AuthStore>();
    let history = use_navigator().unwrap();

    let error_state = use_state(|| None::<String>);

    only_if_not_logged_in!(dispatch, history);

    let cloned_dispatch = dispatch.clone();
    let cloned_error = error_state.clone();
    let custom_form_submit = Callback::from(move |data: login_form::Data| {
        let inner_dispatch = cloned_dispatch.clone();
        let cloned_error = cloned_error.clone();
        let history = history.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let cloned_error = cloned_error.clone();
            let response =
                api::api_login(data.username.to_string(), data.password.to_string()).await;
            match response {
                Ok(logged_in) => match logged_in {
                    ResponseLoginUser::Accepted { token, .. } => {
                        inner_dispatch.set(AuthStore {
                            token: Some(token),
                            username: None,
                        });
                        cloned_error.set(None);
                        history.push(&MainRoute::Home);
                    }
                    ResponseLoginUser::Incorrect { .. } => {
                        cloned_error.set(Some(
                            "Login failed because of incorrect credentials".to_string(),
                        ));
                        log::warn!("Error message on incorrect credentials")
                    }
                },
                Err(err) => {
                    cloned_error.set(Some(
                        "Login failed, maybe the connection to server failed?".to_string(),
                    ));
                    log::error!("Error message on failed login: {err:?}")
                }
            }
        });
    });

    let current_error = (*error_state).clone();

    html!(<Page title={"Please login"} home_icon=false>
    <LoginForm onsubmit={custom_form_submit}/>
    {
        if let Some(current_error) = current_error {
            html!(
                <div class="error-msg">{current_error}</div>
            )
        } else {
            html!()
        }
    }
</Page>)
}
