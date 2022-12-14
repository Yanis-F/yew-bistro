use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::eval::evaluate;

pub enum Message {
    Nothing,
    DoCompute,
    SetInputExpr(String),
    SetInputBase(String),
}

pub struct Bistro {
    input_base: String,
    input_expr: String,
    result: String,
}

impl Component for Bistro {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Nothing => {}
            Message::SetInputExpr(expr) => self.input_expr = expr,
            Message::SetInputBase(base) => self.input_base = base,

            Message::DoCompute => {
            	match evaluate(&self.input_expr, &self.input_base) {
					Ok(result) => self.result = result,
					Err(e)     => self.result = format!("ERROR: {}", e)
				}
			}
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let update_input_expr = link.callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();

            Message::SetInputExpr(input.value())
        });
        let update_input_base = link.callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();

            Message::SetInputBase(input.value())
        });
        let do_compute_when_press_enter = link.callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Message::DoCompute
            } else {
                Message::Nothing
            }
        });

        let launch_computation = link.callback(|_| Message::DoCompute);

        html! {
            <div>
                <span>
                    { "base:" }
                    <input value={self.input_base.clone()} oninput={update_input_base} />
                </span>
                <br/>
                <input value={self.input_expr.clone()} oninput={update_input_expr} onkeyup={do_compute_when_press_enter} />
                <button onclick={launch_computation}>{"Compute"}</button>
                <br/>
                <span class={classes!("result")}>{ self.result.clone() }</span>
            </div>
        }
    }
}

impl Default for Bistro {
    fn default() -> Self {
        Self {
            result: String::new(),
            input_expr: String::new(),
            input_base: "0123456789".to_string(),
        }
    }
}
