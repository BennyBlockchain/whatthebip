use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq)]
pub struct BipItem {
    pub name: String,
    pub resources: Vec<Resource>
}

#[derive(PartialEq)]
pub struct Resource {
    pub title: String,
    pub site: String,
    pub link: String   
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub bip: BipItem
}

pub struct Bip;

impl Component for Bip {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p class="h4">{ctx.props().bip.name.clone()}</p>
                { ctx.props().bip.resources.iter().map(|bip| html! {
                    <li class="px-4">
                        <a class="span" href={bip.link.clone()}>
                            {format!("{}", bip.title)}
                        </a>
                    </li>
                }).collect::<Html>()}
            </>
        }
    }
}