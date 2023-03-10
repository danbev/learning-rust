use yew::prelude::*;

use firstcomponent::FirstComponent;
mod firstcomponent;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div class="container">
            <h1>{"Yew Example Web App"}</h1>
            <div><FirstComponent value={1}/></div>
        </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // for debug level you might have to switch the level in Chrome Devtools
    // to 'verbose' to actually see the logging.
    log::info!("App is starting");
    yew::Renderer::<App>::new().render();
}
