//! Error types

use crate::ffi::MeshLabResult;
use std::ffi::CStr;
use std::fmt;

pub type Result<T> = std::result::Result<T, MeshLabError>;

#[derive(Debug, Clone)]
pub enum MeshLabError {
    NullPointer,
    InvalidParameter(String),
    FileNotFound(String),
    LoadFailed(String),
    SaveFailed(String),
    NoMesh,
    Unknown(String),
}

impl fmt::Display for MeshLabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshLabError::NullPointer => write!(f, "Null pointer"),
            MeshLabError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            MeshLabError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            MeshLabError::LoadFailed(msg) => write!(f, "Failed to load: {}", msg),
            MeshLabError::SaveFailed(msg) => write!(f, "Failed to save: {}", msg),
            MeshLabError::NoMesh => write!(f, "No mesh available"),
            MeshLabError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for MeshLabError {}

pub(crate) fn check_result(result: MeshLabResult) -> Result<()> {
    match result {
        MeshLabResult::Success => Ok(()),
        MeshLabResult::ErrorNullPointer => Err(MeshLabError::NullPointer),
        MeshLabResult::ErrorInvalidParam => {
            let msg = get_last_error().unwrap_or_else(|| "Invalid parameter".to_string());
            Err(MeshLabError::InvalidParameter(msg))
        }
        MeshLabResult::ErrorFileNotFound => {
            let msg = get_last_error().unwrap_or_else(|| "File not found".to_string());
            Err(MeshLabError::FileNotFound(msg))
        }
        MeshLabResult::ErrorLoadFailed => {
            let msg = get_last_error().unwrap_or_else(|| "Load failed".to_string());
            Err(MeshLabError::LoadFailed(msg))
        }
        MeshLabResult::ErrorSaveFailed => {
            let msg = get_last_error().unwrap_or_else(|| "Save failed".to_string());
            Err(MeshLabError::SaveFailed(msg))
        }
        MeshLabResult::ErrorNoMesh => Err(MeshLabError::NoMesh),
        MeshLabResult::ErrorUnknown => {
            let msg = get_last_error().unwrap_or_else(|| "Unknown error".to_string());
            Err(MeshLabError::Unknown(msg))
        }
    }
}

fn get_last_error() -> Option<String> {
    unsafe {
        let ptr = crate::ffi::meshlab_last_error();
        if ptr.is_null() {
            None
        } else {
            CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_string())
        }
    }
}
