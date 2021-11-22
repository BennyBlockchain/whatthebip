use yew::{Component, Context, html, Html, Properties};
use crate::bip::Resource;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub resources: Vec<Resource>,
}

pub struct ResourceList;

impl Component for ResourceList {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            {ctx.props().resources.iter().map(|resource| html! {
                <li>
                <a href={resource.link.clone()}>{format!("{}", resource.title)}</a>
                </li>
            }).collect::<Html>()}
        }    
    }
}