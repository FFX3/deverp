use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Default)]
struct LogoutArgs { }

#[component]
pub fn Logout() -> impl IntoView {

    let logout = move |_ev: MouseEvent| {
        spawn_local(async move {
            let args = to_value(&LogoutArgs::default()).unwrap();
            invoke("logout", args).await;
        });
    };

    view! {
        <button on:click=logout>"Logout"</button>
    }
}
