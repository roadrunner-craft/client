use crate::utils::Bindable;

use gl::types::{GLsizeiptr, GLuint};
use math::vector::{Vector2, Vector3};
use std::mem;
use std::ptr;
use std::vec::Vec;

#[derive(Default, Debug)]
pub struct Model {
    vao: GLuint,
    vbo_count: GLuint,
    index_count: usize,
    buffers: Vec<GLuint>,
}

impl Model {
    pub fn new(vertices: &Vec<Vector3>, indices: &Vec<GLuint>) -> Model {
        let mut model = Model {
            vao: 0,
            vbo_count: 0,
            index_count: indices.len(),
            buffers: Vec::new(),
        };

        unsafe {
            gl::GenVertexArrays(1, &mut model.vao);
        }

        model.add_vbo(vertices);
        model.add_ebo(indices);
        model
    }

    #[allow(dead_code)]
    pub fn add_uv(&mut self, uv: &Vec<Vector2>) {
        self.add_vbo(&uv);
    }

    pub fn index_count(&self) -> usize {
        self.index_count
    }

    pub fn add_vbo<T>(&mut self, data: &Vec<T>) {
        let mut vbo: GLuint = 0;

        self.bind();

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
                (mem::size_of::<T>() / 4) as i32,
                gl::FLOAT,
                gl::FALSE,
                0,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(self.vbo_count);
        }

        self.buffers.push(vbo);
        self.vbo_count += 1;
    }

    fn add_ebo(&mut self, indices: &Vec<GLuint>) {
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

        self.buffers.push(ebo);
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
