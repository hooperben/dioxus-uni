use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        SwapForm {}
    }
}

#[component]
pub fn SwapForm() -> Element {
    let mut pool_address = use_signal(|| "".to_string());
    let mut token_in = use_signal(|| String::new());
    let mut token_out = use_signal(|| String::new());
    let mut amount_in = use_signal(|| String::new());

    let mut name = use_signal(|| "bob".to_string());

    rsx! {
        input {
            class: "text-black",
            // we tell the component what to render
            value: "{name}",
            // and what to do when the value changes
            oninput: move |event| name.set(event.value())
        }
    }
}
