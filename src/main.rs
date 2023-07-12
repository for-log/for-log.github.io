mod components;
mod particle;

use components::*;
use leptos::*;


fn main() {
    mount_to_body(|cx| view! {cx,
        <Canvas />
        <Header />
        <Home />
        <About />
        <Works />
    })
}