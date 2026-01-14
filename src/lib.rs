use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn vue_compile_template(template_str: *const c_char) -> *mut c_char;
    fn vue_compile_batch(templates_json: *const c_char) -> *mut c_char;
    fn vue_free_string(ptr: *mut c_char);
}

#[derive(Debug)]
pub struct CompileError(pub String);

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CompileError {}

pub fn compile_template(template: &str) -> Result<String, CompileError> {
    let c_template = CString::new(template)
        .map_err(|e| CompileError(format!("Invalid template string: {}", e)))?;

    let result_ptr = unsafe { vue_compile_template(c_template.as_ptr()) };

    if result_ptr.is_null() {
        return Err(CompileError("Compilation returned null".to_string()));
    }

    let result_str = unsafe {
        let c_str = CStr::from_ptr(result_ptr);
        let s = c_str.to_string_lossy().into_owned();
        vue_free_string(result_ptr);
        s
    };

    if result_str.starts_with("ERROR: ") {
        return Err(CompileError(result_str[7..].to_string()));
    }

    Ok(result_str)
}

pub fn compile_batch(templates: &[&str]) -> Result<Vec<String>, CompileError> {
    let json = serde_json::to_string(templates)
        .map_err(|e| CompileError(format!("JSON error: {}", e)))?;

    let c_json = CString::new(json)
        .map_err(|e| CompileError(format!("Invalid JSON: {}", e)))?;

    let result_ptr = unsafe { vue_compile_batch(c_json.as_ptr()) };

    if result_ptr.is_null() {
        return Err(CompileError("Batch compilation returned null".to_string()));
    }

    let result_str = unsafe {
        let c_str = CStr::from_ptr(result_ptr);
        let s = c_str.to_string_lossy().into_owned();
        vue_free_string(result_ptr);
        s
    };

    let results: Vec<String> = serde_json::from_str(&result_str)
        .map_err(|e| CompileError(format!("Failed to parse results: {}", e)))?;

    Ok(results)
}
