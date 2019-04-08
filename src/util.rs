use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlRenderingContext as GL;

pub fn load_texture_image(gl: Rc<WebGlRenderingContext>, src: &'static str) {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);

	web_sys::console::log_2(&"loading texture...".into(), &src.into());
    let onload = Closure::wrap(Box::new(move || {
        let texture = gl.create_texture();

        //gl.active_texture(GL::TEXTURE0);

        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());

        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &image_clone.borrow(),
        )
        .expect("Texture image 2d");

		web_sys::console::log_2(&"loaded texture:".into(), &src.into());
    }) as Box<dyn Fn()>);

    let image = image.borrow_mut();

    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_src(src);

    onload.forget();
}