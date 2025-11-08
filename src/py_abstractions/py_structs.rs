use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;

use pyo3_stub_gen::derive::* ;


macro_rules! make_basic_pyclass {
    ($name:ident, { $($field:ident : $ty:ty),* $(,)? }) => {
        #[gen_stub_pyclass]
        #[pyclass]
        #[derive(Clone, PartialEq, Debug)]
        pub struct $name {
            $(#[pyo3(get, set)]
            pub $field: $ty),*
        }
        #[gen_stub_pymethods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new($($field: $ty),*) -> Self {
                Self { $($field),* }
            }

            pub fn __repr__(&self) -> String {
                format!(
                    concat!(stringify!($name), "(", $(stringify!($field), "={:?}, ",)* ")"),
                    $(self.$field),*
                )
            }
        }
    };
}


make_basic_pyclass!(DVec2, { x: f32,y: f32});
make_basic_pyclass!(DVec3, { x: f32,y: f32,z: f32});
make_basic_pyclass!(DVec4, { x: f32,y: f32,z: f32,w: f32});
make_basic_pyclass!(Circle, {x: f32,y: f32,r: f32});
make_basic_pyclass!(Quat, {x: f32,y: f32,z: f32, w: f32});
make_basic_pyclass!(Rect, {x: f32,y: f32,w: f32, h: f32});
make_basic_pyclass!(DMat2, {x_axis: DVec2, y_axis: DVec2});
make_basic_pyclass!(DMat3, {x_axis: DVec3, y_axis: DVec3, z_axis: DVec3});
make_basic_pyclass!(DMat4, {x_axis: DVec4, y_axis: DVec4, z_axis: DVec4, w_axis: DVec4});