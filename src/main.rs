mod components;

use components::*;
use leptos::*;


fn main() {
    mount_to_body(|cx| view! {cx,
        <Header />
        <Home />
        <About />
        <Works />
    })
}