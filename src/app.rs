#![allow(non_snake_case)]
use dioxus::{prelude::*};
use web_sys;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen)]
extern "C" {
    fn tauri_listen(event: &str, cb: &js_sys::Function);
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

    use_effect(||{
        let cb = Closure::<dyn FnMut(js_sys::Object)>::wrap(Box::new(|evt| {
            if let Ok(payload) = js_sys::Reflect::get(&evt, &"payload".into()) {
                if let Some(theme) = payload.as_string() {
                    if let Some(doc) = web_sys::window().unwrap().document() {
                        if let Some(root_el) = doc.document_element() {
                            match theme.as_str() {
                                "dark" => _ = root_el.set_attribute("data-theme", "dark"),
                                "light" => _ = root_el.set_attribute("data-theme", "light"),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }));

        tauri_listen("theme_changed", cb.as_ref().unchecked_ref());
        cb.forget();
    });

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