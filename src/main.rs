use leptos::{*, leptos_dom::{logging, window}};

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let empty_string: String = String::from("");
    let (address, set_address) = create_signal(empty_string.clone());

    fn connect_wallet() -> String {
        let addr: String = String::from("0xTako");
        addr
    }

    view! {
        <h1>"Laptos Wallet Connect"</h1>
        <br/>
        <button
            on:click=move |_| {
                set_address.set(connect_wallet());
                logging::console_log(&address.get_untracked());
            }
        >
            "Connect Wallet"
        </button>
        <br/>
        <h3>
            {address}
        </h3>
    }
}
