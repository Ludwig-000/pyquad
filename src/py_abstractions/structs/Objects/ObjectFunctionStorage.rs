use std::sync::Mutex;
use std::sync::MutexGuard;

use slotmap::SlotMap;
/// stores functions to be executed by Python each frame.
/// 
/// 
/// 
use slotmap::{DefaultKey, DenseSlotMap};
use pyo3::prelude::*;
use pyo3::ffi;
use pyo3::types::*;
use std::sync::{OnceLock};
use slotmap::{new_key_type};


static FUN_STORAGE: OnceLock<Mutex<FunctionStorage>> = OnceLock::new();

pub fn get_fun_storage() -> MutexGuard<'static, FunctionStorage> {
    
    let mutex = FUN_STORAGE.get_or_init(|| {
        Mutex::new(FunctionStorage::new())
    });
    
    mutex.lock().unwrap()
}




new_key_type! { pub struct FunctionKey; }


pub struct FunctionStorage{
    map: SlotMap<FunctionKey, usize>,
    values: Vec<(usize, // raw object pointer as function input.
         Py<PyAny>,     // owned function
         FunctionKey)>, // reverse lookup
}
impl FunctionStorage {
    pub fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            values: Vec::new(),
        }
    }

    pub fn add(&mut self, target: Bound<'_, PyAny>, func: Py<PyAny>) -> FunctionKey {
        let ptr = target.as_ptr() as usize;
        let index = self.values.len();
        
        // 1. Create a stable key pointing to the end of the vector
        let key = self.map.insert(index);
        
        // 2. Store the data and the key (needed for swap_remove updates)
        self.values.push((ptr, func, key));
        key
    }

    pub fn remove(&mut self, key: FunctionKey) {
        if let Some(index) = self.map.remove(key) {
            // Standard O(1) swap_remove
            self.values.swap_remove(index);

            // If we didn't remove the very last element, the element that was 
            // at the end moved to 'index'. we must update its location in the map.
            if index < self.values.len() {
                let (_, _, moved_key) = &self.values[index];
                if let Some(idx_ref) = self.map.get_mut(*moved_key) {
                    *idx_ref = index;
                }
            }
        }
    }

    
    pub fn execute_all(&self, py: Python<'_>) -> PyResult<()> {
        for (ptr_address, callback, _) in self.values.iter() {
            
            unsafe {
                let raw_ptr = *ptr_address as *mut ffi::PyObject;
                
                let target_bound = Bound::from_borrowed_ptr(py, raw_ptr);
                let func_bound = callback.bind(py);
                
                if let Err(e) = func_bound.call1((target_bound,)) {
                    self.report_error(py, &e, func_bound);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    
    #[cfg(false)]
    /// Should massively speed up execution in theory, if free threading is enabled.
    /// for now, this is still slower than linear execution
    pub fn execute_all(&self, py: Python<'_>) -> PyResult<()> {
        use rayon::prelude::*;
        
        const BATCH_SIZE: usize = 64;

        py.detach(|| {
            self.values.par_chunks(BATCH_SIZE).for_each(|chunk| {
                
                let err=Python::attach(|py| {
                    for (ptr_address, callback, _) in chunk {
                        unsafe {
                            let raw_ptr = *ptr_address as *mut ffi::PyObject;
                            
                            let target_bound = Bound::from_borrowed_ptr(py, raw_ptr);
                            let func_bound = callback.bind(py);
                            
                            if let Err(e) = func_bound.call1((target_bound,)) {
                                self.report_error(py, &e, func_bound);
                                return Err(e);
                            }
                        }
                    }
                    Ok(())
                });
            });
        });

        Ok(())
    }
}



impl FunctionStorage {
    fn report_error(&self, py: Python<'_>, err: &PyErr, func: &Bound<'_, PyAny>) {
        println!("\n--- SCRIPT ERROR ---");

        let func_name = func.getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_else(|_| "unknown_func".into());

        if let Some(tb) = err.traceback(py) {
            let mut last_tb = tb.into_any();

            while let Ok(next) = last_tb.getattr("tb_next") {
                if next.is_none() { break; }
                last_tb = next;
            }

            let lineno: i32 = last_tb.getattr("tb_lineno").and_then(|l| l.extract()).unwrap_or(0);
            
            if let Ok(frame) = last_tb.getattr("tb_frame") {
                if let Ok(code) = frame.getattr("f_code") {
                    if let Ok(filename) = code.getattr("co_filename").and_then(|f| f.extract::<String>()) {
                        
                        println!("Function: {}", func_name);
                        println!("Location: {} (Line {})", filename, lineno);

                        if let Ok(linecache) = py.import("linecache") {
                            if let Ok(line) = linecache.call_method1("getline", (filename, lineno)) {
                                println!("Code:     > {}", line.to_string().trim());
                            }
                        }
                    }
                }
            }
        }

        println!("Error:    {}", err.value(py));
        println!("-----------------------\n");
    }
}