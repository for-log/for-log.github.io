use leptos::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Event;

const HEADER_MAIN_TEXT: &str = "Forder Portfolio";

//WARNING: in right order
const HEADER_LINKS: &[(&str, &str)] = &[
    ("Home", "#home"),
    ("About", "#about"),
    ("Works", "#works")
];

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let (is_active, set_is_active) = create_signal(cx, false);
    let (active_menu, set_active_menu) = create_signal(cx, HEADER_LINKS[0].1);


    create_effect(cx, move |_| {
        let window_element = window();
        let window_cloned = window_element.clone();
        let callback = Closure::wrap(Box::new(move |_: Event| {
            let Ok(scroll_y) = window_cloned.scroll_y() else {
                return;
            };

            let condition = scroll_y > 0.0;
            set_is_active.set(condition);

            let document = window_cloned.document().unwrap();
            let Some((_, path)) = HEADER_LINKS.iter().rev().find(|(_, path)| {
                let item = document.query_selector(path).unwrap().unwrap();
                let rect = item.get_bounding_client_rect();

                rect.y() <= scroll_y
            }) else {
                return;
            };

            set_active_menu.set(path);
        }) as Box<dyn FnMut(_)>);
        let _ = window_element.add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref());
        callback.forget();
    });
    view! {cx, 
        <header class={move || if is_active.get() {"active"} else {""}}>
            <a href="#home"><h1>{HEADER_MAIN_TEXT}</h1></a>

            <ul>
                {HEADER_LINKS.iter().map(|(name, path)| view! { cx,
                    <li class={move || if active_menu.get() == *path {"active"} else {""}}><a href={*path}>{*name}</a></li>
                }).collect_view(cx)}
            </ul>
        </header>
    }
}