#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn App() -> Element {
    let mut name = use_signal(|| String::new());
    let mut greet_msg = use_signal(|| String::new());

    let greet = move |_: FormEvent| async move {
        if name.read().is_empty() {
            return;
        }

        let name = name.read();
        let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();

        let new_msg = invoke("greet", args).await.as_string().unwrap();
        greet_msg.set(new_msg);
    };

    rsx! {
        link { rel: "stylesheet", href: "styles.css" }
        main {
            class: "container",
            h1 { "YuLauncherV2" }

            /* div {
                class: "row",
                a {
                    href: "https://tauri.app",
                    target: "_blank",
                    img {
                        src: "/tauri.svg",
                        class: "logo tauri",
                         alt: "Tauri logo"
                    }
                }
                a {
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    img {
                        src: "/dioxus.png",
                        class: "logo dioxus",
                        alt: "Dioxus logo"
                    }
                }
            } */

            form {
                class: "row",
                onsubmit: greet,
                input {
                    id: "greet-input",
                    placeholder: "名前を入力...",
                    value: "{name}",
                    oninput: move |event| name.set(event.value())
                }
                button { r#type: "submit", "Greet" }
            }
            p { "{greet_msg}" }
        }
    }
}