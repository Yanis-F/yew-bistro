use yew::prelude::*;

use crate::bistro::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "Bistromatic" }</h1>
                <div>
                    <Bistro />
                </div>
            </>
        }
    }
}
