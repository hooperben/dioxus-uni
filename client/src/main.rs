use dioxus::{logger::tracing, prelude::*};

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

#[derive(serde::Deserialize)]
struct UniV2API {
    pool: String,
    src: String,
    dst: String,
    amount_out: String,
}

#[component]
pub fn SwapForm() -> Element {
    let mut pool_address = use_signal(|| "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852".to_string());
    let mut token_in = use_signal(|| "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string());
    let mut token_out = use_signal(|| "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string());
    let mut amount_in = use_signal(|| "10000000".to_string());

    let mut expected_output = use_signal(|| "".to_string());

    let get_quote = move |_| async move {
        expected_output.set("".to_string());

        let url = format!(
            "https://uni-v2.hooper.link/estimate?pool={}&src={}&dst={}&src_amount={}",
            pool_address, token_in, token_out, amount_in
        );

        // In a real app, you would make the HTTP request here using reqwest or gloo-net
        // For now, we'll just log the URL
        tracing::info!("Rendering app!");
        tracing::info!("{}", url);

        let response = reqwest::get(url)
            .await
            .unwrap()
            .json::<UniV2API>()
            .await
            .unwrap();

        tracing::info!("{}", response.amount_out);

        expected_output.set(response.amount_out);
    };

    rsx! {
        div { class: "flex flex-col gap-4 p-4 max-w-md mx-auto",
            h2 { class: "text-xl font-bold mb-4", "Uni V2 Expected Output Calculator" }

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

            label { "Amount In (as exp):" }
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

            if expected_output.to_string() != "".to_string() {
                div {
                    class: "flex w-full bg-blue-400 rounded-full text-black p-4",
                     "Expected Output: {expected_output}"

                }
            }
        }
    }
}
