use super::py_structs::*;
use crate::py_abstractions::structs::Textures_and_Images::*;
use macroquad::prelude as mq;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 
use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*};


use std::sync::mpsc;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::*;

use crate::COMMAND_QUEUE;
use crate::Command;

#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_position() -> PyResult<(f32,f32)> {
    let (sender, receiver) = mpsc::sync_channel(1);
    COMMAND_QUEUE.push(Command::GetMousePosition(sender));

    match receiver.recv() {
        Ok(pos) => Ok(pos),
        Err(_) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to receive Mouse Position")),
    }

}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn set_cursor_grab(option: bool)  {

    COMMAND_QUEUE.push(Command::SetCursorGrab(option));
    
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn show_mouse(option: bool) {
    
    COMMAND_QUEUE.push(Command::ShowMouse(option));
    
}

