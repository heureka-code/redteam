use std::ops::Deref;

use yew::prelude::*;

use crate::components::TextInput;

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub username: AttrValue,
    pub password: AttrValue,
}

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(LoginForm)]
pub fn login_form(Props { onsubmit }: &Props) -> Html {
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_changed = move |username: String| {
        let mut data = cloned_state.deref().clone();
        data.username = username.into();
        cloned_state.set(data);
    };

    let cloned_state = state.clone();
    let password_changed = move |password: String| {
        let mut data = cloned_state.deref().clone();
        data.password = password.into();
        cloned_state.set(data);
    };

    let cloned_onsubmit = onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let value = cloned_state.deref().clone();

        cloned_onsubmit.emit(value);
    });

    html!(<form onsubmit={onsubmit}>
        <TextInput name={"username"} placeholder={"username"} onchange={username_changed} />
        <TextInput name={"password"} input_type={"password"} placeholder={"password"} onchange={password_changed}/>
        <input name={"loginBtn"} type={"submit"}/>
    </form>)
}
