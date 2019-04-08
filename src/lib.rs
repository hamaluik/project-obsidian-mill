#[macro_use]
extern crate specs_derive;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext};
use std::cell::RefCell;
use std::rc::Rc;
use specs::prelude::*;

mod components;
mod systems;
mod util;
mod texturepacker;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[derive(Default)]
pub struct DeltaTime(f64);

#[derive(Default)]
pub struct ScreenSize(bool);

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    let crc = Rc::new(context);

    // load the image
    self::util::load_texture_image(crc.clone(), "/assets/spritesheet.png");

    // setup ecs
    let mut world = World::new();
    world.add_resource(DeltaTime(0.05));
    world.add_resource(ScreenSize(true));
    world.register::<components::Transform>();
    world.register::<components::Sprite>();

    world.create_entity()
        .with(components::Transform{ x: 0.0, y: 0.0 })
        .with(components::Sprite{ u: 0.0, v: 0.5, w_uv: 0.5, h_uv: 0.5 })
        .build();

    let mut update_dispatcher = DispatcherBuilder::new()
        //.with(systems::HueShift, "hueshift", &[])
        .build();
    let mut render_dispatcher = DispatcherBuilder::new()
        .with_thread_local(systems::RenderSystem::new(crc)?)
        .build();

    let mut last_t: f64 = 0.0;
    let mut last_size: (u32, u32) = (canvas.width(), canvas.height());
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |t: f64| {
        // update timing
        let dt: f64 = t - last_t;
        last_t = t;
        {
            let mut delta = world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt / 1000.0);
        }

        // detect resize
        {
            let mut ss = world.write_resource::<ScreenSize>();
            *ss = ScreenSize(canvas.width() != last_size.0 || canvas.height() != last_size.1);
            last_size.0 = canvas.width();
            last_size.1 = canvas.height();
        }

        update_dispatcher.dispatch(&world.res);
        render_dispatcher.dispatch_thread_local(&world.res);
        world.maintain();

        // again!
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut(f64)>));
    request_animation_frame(g.borrow().as_ref().unwrap());
    
    Ok(())
}
