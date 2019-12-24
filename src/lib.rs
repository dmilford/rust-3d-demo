extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_funcs;
mod constants;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct DougsClient {
    gl: WebGlRenderingContext,
    _program_color_2d: programs::Color2D,
    _program_color_2d_gradient: programs::Color2DGradient,
    program_graph_3d: programs::Graph3D,
}

#[wasm_bindgen]
impl DougsClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self {
            _program_color_2d: programs::Color2D::new(&gl),
            _program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            program_graph_3d: programs::Graph3D::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        // self.program_color_2d.render(
        //     &self.gl,
        //     curr_state.control_bottom,
        //     curr_state.control_top,
        //     curr_state.control_left,
        //     curr_state.control_right,
        //     curr_state.canvas_height,
        //     curr_state.canvas_width,
        // );

        // self.program_color_2d_gradient.render(
        //     &self.gl,
        //     curr_state.control_bottom + 20.,
        //     curr_state.control_top - 20.,
        //     curr_state.control_left + 20.,
        //     curr_state.control_right - 20.,
        //     curr_state.canvas_height,
        //     curr_state.canvas_width,
        // );

        self.program_graph_3d.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
            curr_state.rotation_x_axis,
            curr_state.rotation_y_axis,
            &common_funcs::get_updated_3d_y_values(curr_state.time),
        );
    }
}