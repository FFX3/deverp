use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SignUpArgs<'a> {
    email: &'a str,
    password: &'a str,
}

#[component]
pub fn SignUp() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    let update_email = move |ev| {
        let v = event_target_value(&ev);
        set_email.set(v);
    };

    let update_password = move |ev| {
        let v = event_target_value(&ev);
        set_password.set(v);
    };

    let sign_up = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let email = email.get_untracked();
            if email.is_empty() {
                return;
            }
            let password = password.get_untracked();
            if password.is_empty() {
                return;
            }

            let args = to_value(&SignUpArgs { email: &email, password: &password }).unwrap();
            invoke("sign_up", args).await;
        });
    };

    view! {
        <form class="row" on:submit=sign_up>
            <input
                id="email-input"
                placeholder="Username..."
                on:input=update_email
            />
            <input
                id="password-input"
                placeholder="Password..."
                type="password"
                on:input=update_password
            />
            <button type="submit">"Sign Up"</button>
        </form>
    }
}
