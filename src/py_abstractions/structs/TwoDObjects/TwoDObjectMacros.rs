#[macro_export]
macro_rules! implement_Drop2D {
    ($name:ident) => {
        paste::paste! {
            impl Drop for $name {
                fn drop(&mut self) {
                    // function storage MUST be cleaned first, since a function inside fun-storage may rely on the object still living.
                    if let Some(key) = self.function_key{
                        let mut storage = ObjectFunctionStorage::get_fun_storage();
                        storage.remove(key);
                    }
                }
            }
        }

    };
}



#[macro_export]
macro_rules! implement_tick2D {
    ($name:ident,  $py_constructor:expr) => {
        paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {

#[doc = 
"
Add a function to this object, which will automatically be executed each frame.
The function must take the object it is attatched to as an argument.
 
Example:
 
```
...# arguments from outside the scope may be included.
>>>delta_time = 0
>>>def update" $name "(obj: " $name "):
...    obj.rot += Vec3.splat(0.2*delta_time)
...
>>>my" $name " = " $py_constructor "
>>>my" $name ".tick(update" $name ")
...
>>>while True:
...    # dt would have to get updated each frame.
...    delta_time = get_delta_time()
...
...    #'next_frame' runs the update function for every object.
...    next_frame()
```
"
]
                pub fn tick(slf: Bound<'_, Self>, function: Bound<'_,PyAny>)-> PyResult<()>{

                    if !function.is_callable(){
                        return Err(PyRuntimeError::new_err(format!("Attatched object {:?} is not callable.",function)));
                    }

                    let mut storage = ObjectFunctionStorage::get_fun_storage();
                    
                    let func_persistent = function.unbind();
                    let obj  = slf.into_any();

                    let key = storage.add(obj, func_persistent);

                    Ok(())
                }
            }
        }

    };
}

