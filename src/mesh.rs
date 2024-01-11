use std::{os::raw::c_void, ptr::null_mut};

pub struct Mesh {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vao_id: u32,
    vbo_id: u32,
    ibo_id: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        let mut instance = Self {
            vertices,
            indices,
            vao_id: 0,
            vbo_id: 0,
            ibo_id: 0,
        };

        unsafe {
            gl::GenBuffers(1, &mut instance.vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, instance.vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (instance.vertices.len() * std::mem::size_of::<f32>()) as isize,
                &instance.vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::GenBuffers(1, &mut instance.vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, instance.vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (instance.vertices.len() * std::mem::size_of::<f32>()) as isize,
                &instance.vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, null_mut());

            gl::GenBuffers(1, &mut instance.ibo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, instance.ibo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (instance.indices.len() * std::mem::size_of::<u32>()) as isize,
                &instance.indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        instance
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}