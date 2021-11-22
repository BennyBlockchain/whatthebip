use yew::{Component, Context, html, Html, Properties};
use crate::resourcelist::ResourceList;
use std::clone::Clone;
use std::vec::Vec;

#[derive(PartialEq, Debug)]
pub struct BipItem {
    pub name: String,
    pub resources: Vec<Resource>
}

#[derive(PartialEq, Debug)]
pub struct Resource {
    pub title: String,
    pub site: String,
    pub link: String   
}

impl Clone for Resource {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            site: self.site.clone(),
            link: self.link.clone()
        }
    }
}

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    pub bips: Vec<BipItem>
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
            { ctx.props().bips.iter().map(|bip| html! {
                <>
                    <p class="h4">{bip.name.clone()}</p>
                    <ResourceList resources={bip.resources.clone()} />
                </>
            }).collect::<Html>()}
            </>
        }
    }
}