use std::sync::Mutex;
use std::sync::MutexGuard;

/// stores functions to be executed by Python each frame.
/// 
/// 
/// 
use slotmap::{DefaultKey, DenseSlotMap};
use pyo3::prelude::*;
use pyo3::ffi;
use pyo3::types::*;
use std::sync::{OnceLock};

static FUN_STORAGE: OnceLock<Mutex<FunctionStorage>> = OnceLock::new();

pub fn get_fun_storage() -> MutexGuard<'static, FunctionStorage> {
    
    let mutex = FUN_STORAGE.get_or_init(|| {
        Mutex::new(FunctionStorage::new())
    });
    
    mutex.lock().unwrap()
}





pub struct FunctionStorage{
    tasks: DenseSlotMap<DefaultKey, 
    (usize, // raw object pointer, used as function input.
    Py<PyAny>)>, // Owned python function.
}

impl FunctionStorage{
    pub fn new() -> Self {
        Self { tasks: DenseSlotMap::default() }
    }
    
    pub fn add(&mut self, target: Bound<'_, PyAny>, func: Py<PyAny>)-> DefaultKey {
        // Gets the raw pointer without incrementing the refcount
        let ptr = target.as_ptr() as usize;
        let key =self.tasks.insert((ptr, func));
        key
    }

    pub fn remove(&mut self, target: DefaultKey){
        self.tasks.remove(target);
    }

    pub fn execute_all(&self, py: Python<'_>) -> PyResult<()> {

        
        for (ptr_address, callback) in self.tasks.values() {
            
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