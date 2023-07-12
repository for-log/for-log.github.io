use std::{rc::Rc, cell::RefCell};
use leptos::*;
use wasm_bindgen::{JsCast, prelude::Closure};
use crate::particle::Particle;


pub const CANVAS_FONT_SIZE: f64 = 20.0;


fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


#[component]
pub fn Canvas(cx: Scope) -> impl IntoView {
    let canvas_ref = create_node_ref(cx);

    let function: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let function_cloned = function.clone();

    let _window = window();

    let body = _window.document()
        .map(|x| x.body().expect("BODY ERROR"))
        .expect("DOCUMENT ERROR");

    let window_width = body.client_width() as f64;
    let window_height = body.client_height() as f64;

    let mut particles = (0..700)
        .map(|_| Particle::new(window_width, window_height))
        .collect::<Vec<_>>();

    canvas_ref.on_load(cx, move |canvas: HtmlElement<html::Canvas>| {      
        canvas.set_width(window_width as u32);
        canvas.set_height(window_height as u32);

        let context = canvas.get_context("2d")
            .unwrap().unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        *function_cloned.borrow_mut() = Some(Closure::new(move || {
            context.clear_rect(0.0, 0.0, window_width, window_height);
    
            for particle in &mut particles {
                particle.render(&context);
                if particle.update() {
                    *particle = Particle::new(window_width, window_height);
                }
            };
            
            request_animation_frame(function.borrow().as_ref().unwrap());
        }));

        request_animation_frame(function_cloned.borrow().as_ref().unwrap());
    });

    view! { cx,
        <canvas _ref=canvas_ref id="main-canvas"></canvas>
    }
}