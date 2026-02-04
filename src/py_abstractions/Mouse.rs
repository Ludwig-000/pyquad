use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

use crate::engine::PChannel::PChannel;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_position() -> PyResult<(f32,f32)> {
    let (sender, receiver) = PChannel::sync_channel(1);
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

