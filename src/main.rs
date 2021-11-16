mod navbar;
mod bip;

use navbar::Navbar;
use bip::{Bip, BipItem, Resource};
use yew::prelude::*;

#[function_component(WTB)]
fn app() -> Html {
    let bip_item = BipItem {
        name: "BIP39".to_string(),
        resources: vec![
            Resource {
                title: "Twitter thread".to_string(),
                site: "Twitter".to_string(),
                link: "https://twitter.com".to_string(),
            },
            Resource {
                title: "Reddit thread".to_string(),
                site: "Reddit".to_string(),
                link: "https://google.com".to_string()
            }
            ]
        };

    html! {
    <>
        <Navbar />
        <div class="container-fluid">
            <h3 class="pt-4 text-center">{ "A collection of fantastic threads/explainers about ₿itcoin BIPs." }</h3>
            <p class="text-center p-0">{"A Bitcoin Improvement Proposal (BIP) is a formal proposal to change Bitcoin."}</p>
            <p class="text-center">{"You can view the full list of BIPs at "}
                <a target="_blank" href="https://github.com/bitcoin/bips#readme">{"bitcoin/bips"}</a>
            </p>
            <Bip bip={bip_item}/>
        </div>
    </>
}
    
}

fn main() {
    yew::start_app::<WTB>();
}
