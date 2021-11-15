use yew::prelude::*;

#[derive(Clone)]
struct Bip {
    num: i32,
    name: String,
    url: String,
    links: Vec<BipLink>
}

#[derive(Clone)]
struct BipLink {
    title: String,
    link: String
}
#[function_component(App)]
fn app() -> Html {
    let bip_list = vec![
        Bip {
            num: 32,
            name: "BIP32".to_string(),
            url: "https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki".to_string(),
            links: vec![
                BipLink {
                    title: "Twitter".to_string(),
                    link: "https://twitter.com".to_string()
                },
                BipLink {
                    title: "Google".to_string(),
                    link: "https://google.com".to_string()
                }
            ]
        },

    ];

    let bips = bip_list.iter().map(|bip| html!{
        <p>
            {format!("{}: {}", bip.num, bip.name)}
            <a href={bip.url.clone()}>{format!("Link")}</a>
        </p>
    }).collect::<Html>();

    html! {
    <>
        <h1>{ "What the BIP?" }</h1>
        <div>
            <h3>{"Bips:"}</h3>
            {bips}
        </div>
    </>
}
    
}

fn main() {
    yew::start_app::<App>();
}
