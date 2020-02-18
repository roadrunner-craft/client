use crate::math::vector::{v2, v3};
use crate::utils::traits::Bindable;

use gl::types::{GLsizeiptr, GLuint};
use std::mem;
use std::ptr;
use std::vec::Vec;

#[derive(Debug)]
pub struct Model {
    vao: GLuint,
    vbo_count: GLuint,
    indices_count: usize,
    buffers: Vec<GLuint>,
}

// TODO: make sure data is passed around by ref to not copy huge models around
impl Model {
    pub fn new(vertices: Vec<v3>, indices: Vec<GLuint>) -> Model {
        let mut model = Model {
            vao: 0,
            vbo_count: 0,
            indices_count: indices.len(),
            buffers: Vec::new(),
        };

        unsafe {
            gl::GenVertexArrays(1, &mut model.vao);
        }

        model.bind();

        model.add_vbo(3, &vertices);
        model.add_ebo(&indices);
        model
    }

    pub fn new_textured(vertices: Vec<v3>, uv: Vec<v2>, indices: Vec<GLuint>) -> Model {
        let mut model = Model::new(vertices, indices);

        model.add_vbo(2, &uv);
        model
    }

    pub fn get_indices_count(&self) -> usize {
        self.indices_count
    }

    pub fn add_vbo<T>(&mut self, dimension: i32, data: &Vec<T>) {
        let mut vbo: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                mem::transmute(&data[0]),
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                self.vbo_count,
                dimension,
                gl::FLOAT,
                gl::FALSE,
                0,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(self.vbo_count);
        }

        self.vbo_count += 1;
    }

    fn add_ebo(&self, indices: &Vec<GLuint>) {
        let mut ebo: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                mem::transmute(&indices[0]),
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Bindable for Model {
    fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao) }
    }

    fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(self.buffers.len() as i32, mem::transmute(&self.buffers));
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
