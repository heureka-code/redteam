use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MaterialIconProps {
    pub name: String,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(MaterialIcon)]
pub fn material_icon(MaterialIconProps { name, class }: &MaterialIconProps) -> Html {
    html!(
        <span class={classes!("material-symbols-outlined", "notranslate", class.clone())}>{name}</span>
    )
}
