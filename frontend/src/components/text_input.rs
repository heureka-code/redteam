use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub name: AttrValue,
    #[prop_or("".into())]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,
    #[prop_or("text".into())]
    pub input_type: AttrValue,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TextInput)]
pub fn text_input(
    Props {
        name,
        placeholder,
        onchange,
        input_type,
        class,
    }: &Props,
) -> Html {
    let onchange_ = match onchange {
        Some(handle_onchange) => {
            let handle_onchange = handle_onchange.clone();
            Callback::from(move |event: Event| {
                let value = event
                    .target()
                    .unwrap()
                    .unchecked_into::<HtmlInputElement>()
                    .value();
                handle_onchange.emit(value);
            })
        }
        None => Callback::from(move |_event| {}),
    };
    html!(
        <input type={input_type.clone()} class={class.clone()} name={name.clone()} onchange={onchange_} placeholder={placeholder.clone()}/>
    )
}
