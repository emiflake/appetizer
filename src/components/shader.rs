#![allow(dead_code)]

use specs::prelude::*;

use gl::types::*;

use std::fs::File;
use std::io::{BufReader, Read};

use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use std::str;

#[derive(Debug)]
pub enum ShaderError {
    LoadError(std::io::Error),
    CompileError(String),
}

#[derive(Component, Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
#[storage(VecStorage)]
pub struct ShaderComponent(pub u32);

impl ShaderComponent {
    pub fn new(vertex_path: &str, fragment_path: &str) -> std::result::Result<Self, ShaderError> {
        let mut vertex_file = File::open(vertex_path).map_err(ShaderError::LoadError)?;
        let mut vertex_reader = BufReader::new(&mut vertex_file);
        let mut vertex_buf = Vec::new();
        vertex_reader
            .read_to_end(&mut vertex_buf)
            .map_err(ShaderError::LoadError)?;
        let c_str_vert = CString::new(vertex_buf).unwrap();

        let mut fragment_file = File::open(fragment_path).map_err(ShaderError::LoadError)?;
        let mut fragment_reader = BufReader::new(&mut fragment_file);
        let mut fragment_buf = Vec::new();
        fragment_reader
            .read_to_end(&mut fragment_buf)
            .map_err(ShaderError::LoadError)?;
        let c_str_frag = CString::new(fragment_buf).unwrap();

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            // check for shader compile errors
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(ShaderError::CompileError(format!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED {}",
                    str::from_utf8(&info_log).unwrap()
                )));
            }

            // fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            // check for shader compile errors
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(ShaderError::CompileError(format!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    str::from_utf8(&info_log).unwrap()
                )));
            }

            // link shaders
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            // check for linking errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(ShaderError::CompileError(format!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    str::from_utf8(&info_log).unwrap()
                )));
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Self(shader_program))
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.0);
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(
            gl::GetUniformLocation(self.get_id(), name.as_ptr()),
            value as i32,
        );
    }
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.get_id(), name.as_ptr()), value);
    }
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.get_id(), name.as_ptr()), value);
    }
    pub unsafe fn set_vector3(&self, name: &CStr, value: &glm::Vec3) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.get_id(), name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }
    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(
            gl::GetUniformLocation(self.get_id(), name.as_ptr()),
            x,
            y,
            z,
        );
    }
    pub unsafe fn set_mat4(&self, name: &CStr, mat: &glm::Mat4) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.get_id(), name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    pub fn get_id(&self) -> u32 {
        self.0
    }
}
