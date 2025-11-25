use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use crate::engine::PError::PError;
use crate::engine::CoreLoop::*;
use std::collections::binary_heap::PeekMut;
use std::sync::mpsc;



/// Loads a file into a List of Bytes.
#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn load_file(path: &str)-> PyResult<Vec<u8>>{
    match std::fs::read(path) {
        Ok(bytes) => Ok(bytes),
        Err(e) => {
            Err(PError::BasicErr(
                format!("Failed to load file {path}: {e}")
            ).into())
        }
    }
}



/// Loads a file into a List of Bytes.
#[cfg(any(target_arch = "wasm32", target_os = "ios"))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn load_file(path: String)-> PyResult<Vec<u8>>{
    let (tx, rx) = mpsc::sync_channel(1);
    COMMAND_QUEUE.push( Command::LoadFile { path, sender: tx } );
    let res = rx.recv().unwrap();
    return match res{
        Ok(r) => Ok(r),
        Err(e) => Err(e.into()),
    }
}




/// Downloads a file and returning it's raw data.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn download_file(url: &str) -> PyResult<Vec<u8>> {

    let resp = reqwest::blocking::get(url)
        .map_err(|e| PError::BasicErr(format!("Request failed for {url}: {e}")))?;

    let resp = resp.error_for_status()
        .map_err(|e| PError::BasicErr(format!("HTTP error for {url}: {e}")))?;
    
    let bytes = resp.bytes()
        .map_err(|e| PError::BasicErr(format!("Failed to read body for {url}: {e}")))?;
    
    Ok(bytes.to_vec())
}



/// Writes raw data to file.
#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn write_to_file(contents: Vec<u8>, path: String) -> PyResult<()> {
    let res = std::fs::write(path.clone(), contents).map_err(|e|{
        PError::BasicErr(format!("Failed to write to file {path}: {e}"))
    })?;
    Ok(())
}