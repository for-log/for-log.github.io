use js_sys::Date;
use rand::{thread_rng, Rng};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::CANVAS_FONT_SIZE;


#[derive(Clone, Debug)]
pub struct Particle {
    x: f64,
    y: f64,
    font_size: f64,
    max_y: f64,
    value: String,
    speed: f64,
    last_update: f64
}


impl Particle {
    pub fn new(max_x: f64, max_y: f64) -> Self {
        let mut random = thread_rng();
        let value = random.gen_range(0..10).to_string();
        let speed = random.gen::<f64>() * 100.0;
        let x = random.gen_range(0.0..max_x);
        let z: f64 = random.gen();
        let font_size = CANVAS_FONT_SIZE * z;
        let last_update = Date::now();

        Self { x, y: 0.0, font_size, max_y, value, speed, last_update }
    }

    pub fn update(&mut self) -> bool {
        let now = Date::now();
        let elapsed = (now - self.last_update) / 1000.;
        self.y += self.speed * elapsed;
        self.last_update = now;
        self.y > self.max_y + CANVAS_FONT_SIZE
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {
        let color = format!("#f4511e{:02x}", (256.0 - self.y * 256.0 / self.max_y) as usize);
        context.set_fill_style(&JsValue::from_str(&color));

        context.set_font(&format!("{:.0}px Open Sans", self.font_size));
        let _ = context.fill_text(
            &self.value, 
            self.x,
            self.y
        );
    }
}