use yew::prelude::*;

#[function_component(LoadingSpinner)]
pub fn loading_spinner() -> Html {
    html!(
        <div class={classes!("lds-roller", "loading-spinner")}>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
        </div>
    )
}
