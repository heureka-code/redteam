macro_rules! only_if_not_logged_in {
    ($auth_dispatch: expr, $navigator: expr) => {
        // if already logged in -> skip login
        let token = $auth_dispatch.get().token.clone();
        if let Some(token) = token {
            let dispatch = $auth_dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let username = crate::api::api_get_username(token)
                    .await
                    .map(|o| o.username().to_owned());
                dispatch.reduce_mut(|s| {
                    if username.is_none() {
                        s.token = None;
                    }
                    s.username = username;
                });
            });
        } else if $auth_dispatch.get().username.is_some() {
            $auth_dispatch.reduce_mut(|s| s.username = None);
        }
        if $auth_dispatch.get().username.is_some() {
            $navigator.push(&crate::MainRoute::Home);
        }
        // end check
    };
}

macro_rules! protect_function_with_login {
    ($dispatch: expr, $navigator: expr) => {
        let token = $dispatch.get().token.clone();
        let history = $navigator;
        let dispatch = $dispatch.clone();

        if let Some(token) = token.clone() {
            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let username = crate::api::api_get_username(token.clone())
                    .await
                    .map(|o| o.username().to_owned());
                if username != dispatch.get().username.clone() || username.is_none() {
                    dispatch.reduce_mut(|s| {
                        if username.is_none() {
                            s.token = None;
                        }
                        s.username = username;
                    });
                }
            });
        }
        if token.is_none() {
            history.push(&crate::MainRoute::Login)
        }
    };
}

pub(crate) use only_if_not_logged_in;
pub(crate) use protect_function_with_login;
