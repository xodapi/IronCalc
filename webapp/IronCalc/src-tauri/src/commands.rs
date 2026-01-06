//! Native commands for IronCalc Desktop
//! These commands provide native file system access for better performance

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileOpenResult {
    pub success: bool,
    pub path: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub memory_mb: u64,
}

/// Opens an XLSX file using native file dialog
#[tauri::command]
pub async fn open_xlsx_file(path: String) -> Result<FileOpenResult, String> {
    let file_path = PathBuf::from(&path);
    
    if !file_path.exists() {
        return Ok(FileOpenResult {
            success: false,
            path: None,
            error: Some(format!("File not found: {}", path)),
        });
    }

    // Validate it's an xlsx file
    match file_path.extension() {
        Some(ext) if ext == "xlsx" || ext == "icalc" => {
            Ok(FileOpenResult {
                success: true,
                path: Some(path),
                error: None,
            })
        }
        _ => Ok(FileOpenResult {
            success: false,
            path: None,
            error: Some("Invalid file type. Expected .xlsx or .icalc".to_string()),
        }),
    }
}

/// Saves a workbook to XLSX file
#[tauri::command]
pub async fn save_xlsx_file(path: String, data: Vec<u8>) -> Result<FileOpenResult, String> {
    use std::fs::File;
    use std::io::Write;

    let file_path = PathBuf::from(&path);
    
    match File::create(&file_path) {
        Ok(mut file) => {
            match file.write_all(&data) {
                Ok(_) => Ok(FileOpenResult {
                    success: true,
                    path: Some(path),
                    error: None,
                }),
                Err(e) => Ok(FileOpenResult {
                    success: false,
                    path: None,
                    error: Some(format!("Failed to write file: {}", e)),
                }),
            }
        }
        Err(e) => Ok(FileOpenResult {
            success: false,
            path: None,
            error: Some(format!("Failed to create file: {}", e)),
        }),
    }
}

/// Gets system information for diagnostics
#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        memory_mb: 0, // Would need sys-info crate for actual value
    }
}
