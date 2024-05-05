use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub name: AttrValue,
    pub pettype: AttrValue,
}

#[function_component(PettableLine)]
pub fn pettable_line(Props { name, pettype }: &Props) -> Html {
    html!(
        <>
            <div>{name}</div>
            <div>{pettype}</div>
        </>
    )
}
