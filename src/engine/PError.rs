
use crate::py_abstractions::structs::Textures_and_Images::*;
use macroquad::prelude as mq;

use macroquad::text;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 
use pyo3_stub_gen::{derive::gen_stub_pyfunction};

use std::fmt::format;
use std::sync::mpsc;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::*;

use crate::COMMAND_QUEUE;
use crate::Command;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::Color::*;
use crate::py_abstractions::structs::KeyCode::*;
use std::fmt;
use symphonia;
use hound;
// creates an intermediate Error, to collect all custom errors and turn them into PythonErrors
// this turns any error into a runtime error
pub enum PError{

    MacroquadErr(macroquad::Error),
    Pyo3Err(pyo3::PyErr),
    SymphoniaError(symphonia::core::errors::Error),
    HoundError(hound::Error),
    BasicErr(String),

    WithContext(Box<PError>, String),
}
pub struct PyErrorWithMoreContext(pub Box<Option<PError>>, pub String);

impl PError {
    /// adds a string to the end of the error.
    pub fn with_context(self, context: impl Into<String>) -> PError {
        PError::WithContext(Box::new(self), context.into())
    }
}


impl From<PError> for pyo3::PyErr {
    fn from(value: PError) -> pyo3::PyErr {
        regular_extract(value, None)
    }
}


// "extra" is a message added at the end of our error independent of error type.
fn regular_extract(value: PError, extra: Option<&str>) -> pyo3::PyErr {

    let extra = match extra {
         None => "",
         Some(s) => s,
    };

    match value {
        PError::MacroquadErr(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e} {extra}")),
        PError::Pyo3Err(e) => e,
        PError::SymphoniaError(e) => handle_Symphonia_error(e, extra),
        PError::BasicErr(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e} {extra}")),
        PError::HoundError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Hound Error: {e} {extra}")),

        PError::WithContext(err,context ) => {
            let (e,c) = recursively_extract_context(*err, context);
            pyo3::PyErr = regular_extract(e, Some(&c))
        }
    }
}

fn recursively_extract_context(err: PError, mut context: String) -> (PError, String) {
    match err {
        PError::WithContext(inner, ctx) => {
            if !context.is_empty() {
                context.push_str(" "); // add separator
            }
            context.push_str(&ctx);
            recursively_extract_context(*inner, context)
        }
        other => (other, context),
    }
}



// "extra" is a message added at the end of our error independent of error type.
fn handle_Symphonia_error(e: symphonia::core::errors::Error, extra: &str ) -> pyo3::PyErr {
    use symphonia::core::errors::*;
    match e {
        Error::DecodeError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia Decode Error: {e} {extra}")),
        Error::IoError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia IoError: {e} {extra}")),
        Error::LimitError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia LimitError: {e} {extra}")),
        Error::ResetRequired => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia Reset Required Error {extra}")),
        Error::SeekError(e) => {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia SeekError: {:?} {extra}",e))
        },
        Error::Unsupported(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia UnsupportedError: {e} {extra}")),
    }
}


fn unstring(s: Option<String>)-> String{
    match s {
       None => "".to_string(),
       Some(ss) => ss,
    }
}


impl From<macroquad::Error> for PError {
    fn from(value: macroquad::Error) -> PError {
        PError::MacroquadErr(value)
    }
}

impl From<pyo3::PyErr> for PError {
    fn from(value: pyo3::PyErr) -> PError {
        PError::Pyo3Err(value)
    }
}

impl From<symphonia::core::errors::Error> for PError {
    fn from(value: symphonia::core::errors::Error) -> PError {
        PError::SymphoniaError(value)
    }
}

impl From<hound::Error> for PError {
    fn from(value: hound::Error) -> PError {
        PError::HoundError(value)
    }
}