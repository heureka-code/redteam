use std::ops::Deref;

use common::ResponsePettable;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub name_pattern: AttrValue,
}

#[function_component(Pettable)]
pub fn pettable(Props { name_pattern }: &Props) -> Html {
    use super::PettableLine;
    use crate::stores::AuthStore;

    let (_state, dispatch) = use_store::<AuthStore>();

    let name_pattern = name_pattern.to_owned();
    let token = dispatch.get().token.clone();

    let token = if let Some(token) = token {
        token
    } else {
        return html!();
    };
    let pet_state = use_state(|| None::<ResponsePettable>);
    let cloned_pet_state = pet_state.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let pet_state = pet_state.clone();
        let pets = crate::api::api_get_pets(token.to_string(), name_pattern.to_string()).await;
        if pet_state.deref() != &pets {
            pet_state.set(pets);
        }
    });

    let pets = cloned_pet_state
        .as_ref()
        .map(|r| r.owned_pets())
        .unwrap_or([].into());

    html!(
        <div class={classes!("pettable")}>
            <PettableLine key={0} name={"Name"} pettype={"Type"}/>
            {
                pets.into_iter().enumerate().map(|(idx, pet)| {
                    html!(<PettableLine key={idx} name={pet.name().to_string()} pettype={pet.pettype().to_string()}/>)
                }).collect::<Html>()
            }
        </div>
    )
}
