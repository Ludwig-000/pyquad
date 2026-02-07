// for now, we cannot use macros to create pyfunctions, since pyo3-stub-gen does NOT play with multiple impl blocks.
// :((((  i tried
// will keep this code here as a reminder.
#[macro_export]
macro_rules! basic_3D_magic_methods {
    ($name:ident) => {

        #[pymethods]
        impl $name {
            fn __eq__(&self, other: &Self) -> bool {
                self.key == other.key
            }
        
        
            fn __hash__(&self) -> u64 {
                let mut s = std::collections::hash_map::DefaultHasher::new();
                self.key.hash(&mut s);
                s.finish()
            }
        
        
            fn __repr__(&self) -> String {
                let pos = self.pos();
                let rot = self.rot();
                let scale  = self.scale();
                let has_tick_function = if self.function_key == None {false} else {true};
                format!("{}(position={:?}, rotation={:?}, scale={:?}, has_tick_function={})", 
                    stringify!($name), pos, rot, scale, has_tick_function)
            }
        
            fn __str__(&self)-> PyResult<String>{
                let pos = self.pos()?;
                Ok(format!("{} at ({:.2}, {:.2}, {:.2})", 
                    stringify!($name), pos.x, pos.y, pos.z))
            }
        }

    };
}