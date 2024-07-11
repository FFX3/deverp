mod auth;

use leptos::*;
use auth::components::signup::SignUp;
use auth::components::login::Login;
use auth::components::logout::Logout;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <main class="bg-yellow-700">
                <Login />
                <SignUp />
                <Logout />
            </main>
        }
    })
}
