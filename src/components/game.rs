use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{
    function_component, 
    {html::onkeyup::Event}, 
    Html, use_state, 
    use_node_ref, html
};

#[function_component]
pub fn Game() -> Html {
    let canvas_ref = use_node_ref();
    let canvas_ref_cloned = canvas_ref.clone();

    let game_start = use_state(|| false);

    let cb = Closure::wrap(Box::new(move |e: Event| {
        if e.key() != "Enter" || *game_start {
            return;
        }
        let canvas = canvas_ref_cloned
            .cast::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();
        context.rect(30.0, 30.0, 20.0, 20.0);
        context.fill();
    }) as Box<dyn FnMut(_)>);

    let document = web_sys::window().unwrap().document().unwrap();
    let _ = document.add_event_listener_with_callback("keyup", cb.as_ref().unchecked_ref());
    cb.forget();

    html! {
        <canvas width="100%" height="300px" class="game__canvas" ref={canvas_ref}/>
    }
}