use yew::prelude::*;

pub struct Bip;

impl Component for Bip {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Bip
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>
                    {"This is a bip component."}
                    <br/>
                    {"Use it as you wish!"}
                </p>
            </div>
        }
    }
}