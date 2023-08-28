use yew::prelude::*;

use firstcomponent::FirstComponent;
mod firstcomponent;
mod usestate;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div class="container">
            <h1>{"Yew Example Web App"}</h1>
            <div><FirstComponent value={1}/></div>
            <div><usestate::UseState/></div>

            <BrowserRouter>
                <h1>{"Routing example"}</h1>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
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

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/bajja")]
    Bajja,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Bajja)]
fn bajja() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Bajja" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    log::info!("switch: {:?}", routes);
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Bajja => html! {
            <Bajja />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
