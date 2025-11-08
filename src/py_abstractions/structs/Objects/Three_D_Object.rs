use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;

use pyo3_stub_gen::derive::* ;


#[gen_stub_pyclass]
#[pyclass(subclass)]
pub struct Three_D_Object{}