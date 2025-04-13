use dioxus::{logger, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
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

    let get_quote = move |_| {
        let url = format!(
            "http://localhost:1337?pool_address={}&token_in={}&token_out={}&amount_in={}",
            pool_address, token_in, token_out, amount_in
        );

        // In a real app, you would make the HTTP request here using reqwest or gloo-net
        // For now, we'll just log the URL
        println!("Getting quote from: {}", url);

        // Placeholder for the actual HTTP request:
        // use_future(async move {
        //     let resp = reqwest::get(&url).await?;
        //     let text = resp.text().await?;
        //     Ok::<_, reqwest::Error>(text)
        // });
    };

    rsx! {
        div { class: "flex flex-col gap-4 p-4 max-w-md mx-auto",
            h2 { class: "text-xl font-bold mb-4", "Swap Form" }

            label { "Pool Address:" }
            input {
                class: "text-black p-2 border rounded",
                value: "{pool_address}",
                oninput: move |event| pool_address.set(event.value())
            }

            label { "Token In:" }
            input {
                class: "text-black p-2 border rounded",
                value: "{token_in}",
                oninput: move |event| token_in.set(event.value())
            }

            label { "Token Out:" }
            input {
                class: "text-black p-2 border rounded",
                value: "{token_out}",
                oninput: move |event| token_out.set(event.value())
            }

            label { "Amount In:" }
            input {
                class: "text-black p-2 border rounded",
                value: "{amount_in}",
                oninput: move |event| amount_in.set(event.value())
            }

            button {
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-4",
                onclick: get_quote,
                "Get Quote"
            }
        }
    }
}
