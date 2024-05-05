mod api;
mod components;
mod route_switch;
mod routes;
mod stores;

use routes::MainRoute;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    use route_switch::switch_main;
    html!(
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    )
}

fn main() {
    dotenv::dotenv().ok();

    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
