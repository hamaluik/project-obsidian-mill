use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation};
use specs::{Read, ReadStorage, System};

pub struct RenderSystem {
    context: Rc<WebGlRenderingContext>,
    image_location: WebGlUniformLocation,
}

impl RenderSystem {
    pub fn new(context: Rc<WebGlRenderingContext>) -> Result<RenderSystem, JsValue> {
        let vert_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            include_str!("sprite.vert.glsl"),
        )?;
        let frag_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            include_str!("sprite.frag.glsl"),
        )?;
        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let image_location = context.get_uniform_location(&program, "IMAGE").ok_or("failed to find colour loc!")?;
        
        let position_location = context.get_attrib_location(&program, "position") as u32;
        let uv_location = context.get_attrib_location(&program, "uv") as u32;

        let vertices: [f32; 4 * (3+2)] = [
            -1.0, -1.0, 0.0,   0.0, 0.0,
             1.0, -1.0, 0.0,   1.0, 0.0,
             1.0,  1.0, 0.0,   1.0, 1.0,
            -1.0,  1.0, 0.0,   0.0, 1.0,
        ];
        let vertex_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let vertices_location = vertices.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&vertex_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

        let buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::DYNAMIC_DRAW,
        );
        context.vertex_attrib_pointer_with_i32(position_location, 3, WebGlRenderingContext::FLOAT, false, 4*(3+2), 0);
        context.vertex_attrib_pointer_with_i32(uv_location, 2, WebGlRenderingContext::FLOAT, false, 4*(3+2), 4*(3));
        context.enable_vertex_attrib_array(position_location);
        context.enable_vertex_attrib_array(uv_location);

        let indices: [u16; 6] = [
            0, 1, 2,
            2, 3, 0
        ];
        let index_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let indices_location = indices.as_ptr() as u32 / 4;
        let index_array = js_sys::Float32Array::new(&index_buffer)
            .subarray(indices_location, indices_location + indices.len() as u32);

        let buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array,
            WebGlRenderingContext::DYNAMIC_DRAW,
        );

        Ok(RenderSystem {
            context,
            image_location,
        })
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (Read<'a, crate::ScreenSize>, ReadStorage<'a, crate::components::Colour>);

    fn run(&mut self, data: Self::SystemData) {
        let (ss, colour) = data;
        if ss.0 {
            self.context.viewport(0, 0, self.context.drawing_buffer_width(), self.context.drawing_buffer_height());
        }

        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        use specs::Join;
        for colour in colour.join() {
            let colour = hsl::HSL {
                h: colour.hue,
                s: 1.0,
                l: 0.5,
            };
            let rgb = colour.to_rgb();
            let rgb: [f32; 3] = [(rgb.0 as f32) / 255.0, (rgb.1 as f32) / 255.0, (rgb.2 as f32) / 255.0];
            //self.context.uniform3fv_with_f32_array(Some(&self.colour_location), &rgb);
            self.context.uniform1i(Some(&self.image_location), 0);

            // draw!
            //self.context.draw_arrays(
            //    WebGlRenderingContext::TRIANGLES,
            //    0,
            //    3,//(vertices.len() / 3) as i32,
            //);
            self.context.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                6,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0
            );
        }
    }
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
