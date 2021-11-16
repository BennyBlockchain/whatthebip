use yew::{Component, Context, html, Html};

pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-light bg-light">
                <div class="container-fluid">
                    <span class="navbar-brand mb-0 h1">{"What the BIP?"}</span>
                    <div class="d-flex">
                        <span>{"by ₿en Schroth"}</span>
                    </div>
                </div>
            </nav>
        }
    }
}