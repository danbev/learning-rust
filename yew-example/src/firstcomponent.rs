use web_sys::HtmlInputElement;
use yew::{html, Component, Html, InputEvent, Properties, TargetCast};

pub struct FirstComponent {}

#[derive(Properties, PartialEq)]
pub struct FirstComponentProps {
    pub value: i32,
}

impl Component for FirstComponent {
    type Message = ();
    type Properties = FirstComponentProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let value = ctx.props().value;
        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            let val: u32 = match input_el.value().parse() {
                Ok(val) => val,
                Err(err) => {
                    log::error!("error ocurred parsing value: {}", err);
                    0
                }
            };
            log::info!("Input value: {}", val);
        });

        html! {
                <div>
                    <label>{format!("Input value. {}", value)}</label>

                    <input type="number" placeholder="input a number" oninput={on_input} min={value.to_string()} max=10/>
                </div>
        }
    }
}
