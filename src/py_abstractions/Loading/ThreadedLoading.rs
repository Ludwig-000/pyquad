use pyo3::prelude::*;
 
use pyo3_stub_gen::derive::* ;
use pyo3::types::PyDict;
use pyo3::prelude::*;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use std::collections::HashMap;
use reqwest;
use pyo3::PyResult;

use std::thread;
use crate::engine::PError::PError;
use crate::py_abstractions::Loading::Loading::{self as load, download_file, write_to_file};


/// Namespace for static Download-related functions.
#[gen_stub_pyclass]
#[pyclass]
pub struct Download;

#[gen_stub_pymethods]
#[pymethods]
impl Download {
    
    

    /// downloads a ressource file and saves it at the given filepath.
    /// Does nothing if the given filepath already exists.
    #[staticmethod]
    pub fn download_file_and_save(url: String, filepath: String)-> PyResult<()>{
        let data = download_file(&url)?;
        write_to_file(data, filepath)
    }


    #[staticmethod]
    pub fn download_file_and_save_and_load(url: String, filepath: String)-> PyResult<Vec<u8>>{
        let data = download_file(&url)?;
        write_to_file(data.clone(), filepath)?;
        Ok(data)
    }


    
    #[staticmethod]
    fn download_file(url: &str) -> PyResult<Vec<u8>> {
        load::download_file(url)
    }


    #[staticmethod]
    fn make_dict(py: Python<'_>) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("a", 1).unwrap();
        dict.set_item("b", "x").unwrap();
        dict.into()
    }




}


/// validates that the link is an actual link.
fn verify_link(link: &str){

}

fn process_manual_dict(py_dict: &Bound<'_, PyDict>) -> PyResult<Vec<var_file>> {
    let mut users: Vec<var_file> = Vec::new();
    
    for (key, value) in py_dict {
        let var_name: String = key.extract()?;
        let file_path: String = value.extract()?;
        users.push(
            var_file{var_name, file_path}
        );
    }
    Ok(users)
}


fn threaded_map<T, U, F>(
    items: Vec<T>, 
    op: F
) -> PyResult<Vec<U>> 
where 
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> PyResult<U> + Send + Sync + 'static + Clone,
{
    let handles: Vec<_> = items.into_iter().map(|item| {
        let op_clone = op.clone();
        thread::spawn(move || {
            op_clone(item)
        })
    }).collect();
    
    handles.into_iter().map(|handle| {
        
        let inner_result = handle.join().map_err(|panic_payload| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Worker thread panicked: {:?}", panic_payload)
            )
        })?;

        inner_result

    }).collect()
}




/// all the structs to make from_dict work
/// 
/// 
#[derive(FromPyObject)]
struct var_file_url {
    var_name: String,
    file_path: String,
    url: String,
}
#[derive(FromPyObject)]
struct var_file {
    var_name: String,
    file_path: String,
}

#[derive(FromPyObject)]
struct file_url {
    file_path: String,
    url: String,
}