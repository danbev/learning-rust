#[allow(unused_imports)]
#[allow(dead_code)]
use yew::{function_component, html, use_effect, use_state, use_state_eq, Callback, Html};

#[function_component(UseState)]
pub fn state() -> Html {
    use_effect(|| {
        log::info!("use_effect called");
        || {}
    });

    // So this function is called when rending and not creation time as I
    // initially thought.
    log::info!("Rendering UseState component");
    //let counter = use_state(|| 0);
    let counter = use_state_eq(|| 0);
    let _use_state_handle: &yew::UseStateHandle<u32> = &counter;
    log::info!("counter: {}", *counter);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            log::info!("callback counter: {}", *counter);
            // Comment out this to see the difference between use_state and use_state_eq
            //counter.set(*counter + 1)
        })
    };

    html! {
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}
