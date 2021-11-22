mod navbar;
mod bip;
mod bip_list;
mod resourcelist;

use navbar::Navbar;
use bip::Bip;
use yew::prelude::*;
use crate::bip_list::get_bips;

#[function_component(WTB)]
fn app() -> Html {
    let bips = get_bips();

    html! {
    <>
        <Navbar />
        <div class="container-fluid">
            <h3 class="pt-4 text-center">{ "A collection of fantastic threads/explainers about ₿itcoin BIPs." }</h3>
            <p class="text-center p-0">{"A Bitcoin Improvement Proposal (BIP) is a formal proposal to change Bitcoin."}</p>
            <p class="text-center">{"You can view the full list of BIPs at "}
                <a target="_blank" href="https://github.com/bitcoin/bips#readme">{"bitcoin/bips"}</a>
            </p>
           <Bip bips={bips}/>
        </div>
    </>
}
    
}

fn main() {
    yew::start_app::<WTB>();
}
