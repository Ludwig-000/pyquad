//use pyo3::prelude::*;
//use std::fs;
//use std::path::PathBuf;

//#[allow(dead_code)]
//fn save_shader_to_package(py: Python<'_>, filename: &str, data: &[u8]) -> PyResult<PathBuf> {
//    // Fallback path if everything fails
//    let mut fallback_path = std::env::temp_dir().join(filename);

//    // Try to locate the package directory
//    let pkg_dir = (|| -> Option<PathBuf> {
//        let module: PyResult<&PyModule> = py.import("pyquad");
//        let module = module.ok()?;
        
//        let file_path: PyResult<String> = module.getattr("__file__")?.extract();
//        let file_path = file_path.ok()?;
        
//        PathBuf::from(file_path).parent().map(|p| p.to_path_buf())
//    })();

//    // Choose target directory: either the package dir or temp dir
//    let target_dir = pkg_dir.unwrap_or_else(|| {
//        // If we can't find the package dir, use OS temp dir
//        std::env::temp_dir()
//    });

//    let shader_cache_dir = target_dir.join("_shader_cache");

//    // Try to make the directory
//    if let Err(e) = fs::create_dir_all(&shader_cache_dir) {
//        eprintln!("[save_shader_to_package] Failed to create dir {:?}: {}", shader_cache_dir, e);
//        // If we fail, still use fallback path
//        return match fs::write(&fallback_path, data) {
//            Ok(_) => Ok(fallback_path),
//            Err(_) => Ok(PathBuf::new()), // Even if writing fails, return an empty path
//        };
//    }

//    let out_path = shader_cache_dir.join(filename);

//    // Attempt to write the file
//    match fs::write(&out_path, data) {
//        Ok(_) => Ok(out_path),
//        Err(e) => {
//            eprintln!("[save_shader_to_package] Failed to write to {:?}: {}", out_path, e);
//            // Fallback: try temp dir
//            match fs::write(&fallback_path, data) {
//                Ok(_) => Ok(fallback_path),
//                Err(_) => Ok(PathBuf::new()),
//            }
//        }
//    }
//}


//fn cache_shader(py: Python<'_>, shader_data: Vec<u8>, filename: String) -> PyResult<String> {
//    let out_path = save_shader_to_package(py, &filename, &shader_data)?;
//    Ok(out_path.display().to_string())
//}

