// ============================================================================
// main.rs - Backend logic for the Tauri application
//
// This file defines the Rust-side backend for the Tauri desktop app. It:
// - Sets up the main app builder
// - Registers commands (that can be called from the frontend via `invoke`)
// - Runs the core ripgrep command to perform fast text search
// ============================================================================

// Prevents an extra console window from opening in release mode on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;         // To run shell commands (ripgrep)
use tauri::command;                // Attribute to expose functions to JS
use rfd::FileDialog;              // For native file dialogs
use std::fs;
use std::io::Read;
use std::path::Path;

// ----------------------
// Main application entry
// ----------------------
fn main() {
    tauri::Builder::default()                  // Initialize Tauri builder
        .invoke_handler(                       // Register Rust functions exposed to JS
            tauri::generate_handler![search_text, open_folder_dialog, read_file] // Include all commands
        )
        .run(tauri::generate_context!())       // Run the application with Tauri context
        .expect("error while running tauri application"); // Crash if startup fails
}

// -----------------------------------------------------------
// This function is exposed to the frontend via Tauri's `invoke`
// It runs `ripgrep` (rg) to search for `query` in `path`
// -----------------------------------------------------------
#[command]  // Expose this function to the frontend via Tauri
fn search_text(query: String, path: String) -> Result<String, String> {
    // Validate inputs
    if query.trim().is_empty() {
        return Err("Search query cannot be empty".to_string());
    }
    if path.trim().is_empty() {
        return Err("Search path cannot be empty".to_string());
    }

    // First, let's check if the path exists and is accessible
    if !std::path::Path::new(&path).exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    // Execute `rg` command with minimal options first
    let output = Command::new("rg")
        .arg("--line-number")                       // Show line numbers
        .arg("--with-filename")                     // Always show filename
        .arg("--smart-case")                        // Case sensitive only if query contains uppercase
        .arg("--no-ignore")                         // Don't respect .gitignore
        .arg("--hidden")                            // Search hidden files
        .arg("--text")                              // Only search text files
        .arg(&query)
        .arg(&path)
        .output()
        .map_err(|e| format!("Failed to run ripgrep: {}", e))?;

    if output.status.success() {
        let results = String::from_utf8_lossy(&output.stdout);
        
        if results.trim().is_empty() {
            return Err("No results found".to_string());
        }
        
        // Format the results with HTML-like tags for styling
        let formatted = results
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                // Find the last two colons to properly split the line
                let last_colon = line.rfind(':').unwrap_or(0);
                let second_last_colon = line[..last_colon].rfind(':').unwrap_or(0);
                
                if last_colon > 0 && second_last_colon > 0 {
                    let file_name = line[..second_last_colon].trim().replace('\\', "/");
                    let line_num = line[second_last_colon+1..last_colon].trim();
                    let content = line[last_colon+1..].trim();
                    
                    // Skip lines that look like binary data or are too long
                    if content.len() > 1000 || content.chars().any(|c| !c.is_ascii() && !c.is_whitespace()) {
                        return String::new();
                    }
                    
                    // Escape special characters in content for XML-like format
                    let escaped_content = content
                        .replace("&", "&amp;")
                        .replace("<", "&lt;")
                        .replace(">", "&gt;")
                        .replace("\"", "&quot;")
                        .replace("'", "&apos;");
                    
                    format!("<line file=\"{}\" num=\"{}\">{}</line>", file_name, line_num, escaped_content)
                } else {
                    String::new()
                }
            })
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>()
            .join("\n");
            
        if formatted.trim().is_empty() {
            return Err("No results found".to_string());
        }
            
        Ok(formatted)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        if error.trim().is_empty() {
            Err("No results found".to_string())
        } else {
            Err(error.trim().to_string())
        }
    }
}

#[command]
fn open_folder_dialog() -> Option<String> {
    FileDialog::new()
        .set_directory(".")
        .pick_folder()
        .map(|p| p.display().to_string())
}

#[command]
fn read_file(path: String) -> Result<String, String> {
    // Validate path
    if path.trim().is_empty() {
        return Err("File path cannot be empty".to_string());
    }

    let path = Path::new(&path);
    if !path.exists() {
        return Err(format!("File does not exist: {}", path.display()));
    }

    // Check if it's a file
    if !path.is_file() {
        return Err(format!("Path is not a file: {}", path.display()));
    }

    // Try to read the file
    let mut file = fs::File::open(path)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    // Read file content
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Check if the content looks like binary data
    if content.len() > 1_000_000 { // 1MB limit
        return Err("File is too large to preview".to_string());
    }

    // Check if the content contains non-text characters
    if content.chars().any(|c| !c.is_ascii() && !c.is_whitespace()) {
        return Err("File appears to be binary or contains non-text characters".to_string());
    }

    Ok(content)
}
