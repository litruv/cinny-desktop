#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct ClipboardImage {
    data: String, // base64 encoded PNG
    mime_type: String,
}

#[tauri::command]
fn get_clipboard_image() -> Result<Option<ClipboardImage>, String> {
    eprintln!("[Clipboard] get_clipboard_image called");
    
    // First, check what types are available
    let list_output = Command::new("wl-paste")
        .arg("--list-types")
        .output();
    
    if let Ok(output) = &list_output {
        let types = String::from_utf8_lossy(&output.stdout);
        eprintln!("[Clipboard] Available types: {}", types.replace('\n', ", "));
    }
    
    // Try to get image data using wl-paste
    // Try PNG first, then other image formats
    let mime_types = ["image/png", "image/jpeg", "image/bmp", "image/gif"];
    
    for mime_type in &mime_types {
        eprintln!("[Clipboard] Trying MIME type: {}", mime_type);
        
        let output = Command::new("wl-paste")
            .arg("--type")
            .arg(mime_type)
            .output();
        
        match output {
            Ok(result) if result.status.success() && !result.stdout.is_empty() => {
                eprintln!("[Clipboard] Got {} bytes of {}", result.stdout.len(), mime_type);
                let base64_data = STANDARD.encode(&result.stdout);
                return Ok(Some(ClipboardImage {
                    data: base64_data,
                    mime_type: mime_type.to_string(),
                }));
            }
            Ok(result) => {
                let stderr = String::from_utf8_lossy(&result.stderr);
                if !stderr.is_empty() {
                    eprintln!("[Clipboard] wl-paste stderr for {}: {}", mime_type, stderr.trim());
                }
            }
            Err(e) => {
                eprintln!("[Clipboard] wl-paste error for {}: {}", mime_type, e);
            }
        }
    }
    
    eprintln!("[Clipboard] No image found in clipboard");
    Ok(None)
}

fn main() {
    let port = 44548;

    tauri::Builder::default()
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_clipboard_image])
        .run(tauri::generate_context!())
        .expect("error while building tauri application")
}
