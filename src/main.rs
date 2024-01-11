mod renderer;
mod shader;
mod gl_error;
mod mesh;

pub use gl_error::*;
use shader::{Shader, ShaderBuilder};

use std::ptr::null_mut;

use glfw::{Action, Context, Key};
use mesh::Mesh;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, -1.0,
         0.5, -0.5, -1.0,
         0.5,  0.5, -1.0,
        -0.5,  0.5, -1.0,

         0.5,  0.5, -1.0,
         1.5,  0.5, -1.0,
         1.5,  1.5, -1.0,
         0.5,  1.5, -1.0,
    ];

    #[rustfmt::skip]
    let indices: Vec<u32> = vec![
        0,1,2,
        2,3,0,

        4,5,6,
        6,7,4,
    ];

    let mesh = Mesh::new(vertices, indices);

    let shader = ShaderBuilder::default()
        .with_fragment(include_str!("frag.glsl").to_string()).expect("Failed to build shader: fragment sourcing failed")
        .with_vertex(include_str!("vert.glsl").to_string()).expect("Failed to build shader: vertex sourcing failed")
        .build();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        shader.bind();

        mesh.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, null_mut());
        }
        mesh.unbind();

        shader.unbind();
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}