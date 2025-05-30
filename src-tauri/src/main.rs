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
use std::os::windows::process::CommandExt;  // For creation_flags
use std::fs::OpenOptions;
use std::io::Write;
use std::env::temp_dir;  // Add this for temporary directory
use chrono::Local;  // Add this for better timestamp formatting
use dirs;  // Added for home directory
use std::path::Path;

// Embed the ripgrep binary
const RG_BINARY: &[u8] = include_bytes!("../bin/rg.exe");

// ----------------------
// Main application entry
// ----------------------
fn main() {
    // Purpose: Entry point for the Tauri backend.
    // - Sets up the logs directory.
    // - Registers Tauri commands (search_text, open_folder_dialog, read_file).
    // - Starts the Tauri event loop.
    // Used by: Tauri runtime.
    println!("Tauri backend starting...");
    // Create logs directory immediately on startup
    if let Ok(project_dir) = std::env::current_dir() {
        let logs_dir = project_dir.join("logs");
        match fs::create_dir_all(&logs_dir) {
            Ok(_) => println!("Created logs directory at: {}", logs_dir.display()),
            Err(e) => println!("Failed to create logs directory: {}", e),
        }
    }
    
    tauri::Builder::default()
        .invoke_handler(
            tauri::generate_handler![search_text, open_folder_dialog, read_file]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// -----------------------------------------------------------
// This function is exposed to the frontend via Tauri's `invoke`
// It runs `ripgrep` (rg) to search for `query` in `path`
// -----------------------------------------------------------
#[tauri::command]
async fn search_text(query: String, path: String) -> Result<String, String> {
    // Purpose: Runs ripgrep with the given query and path, returns results and stats.
    // Used by: Svelte frontend via invoke('search_text').
    // Special: Parses ripgrep output to separate stats and results for frontend parsing.
    println!("Searching for '{}' in '{}'", query, path);
    
    // Validate path exists
    let path = Path::new(&path);
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    
    // Run ripgrep with stats
    let output = Command::new("rg")
        .arg("--stats")
        .arg("--no-heading")
        .arg("--no-column")
        .arg("--line-number")
        .arg(&query)
        .arg(&path)
        .output()
        .map_err(|e| format!("Failed to execute ripgrep: {}", e))?;
    
    // Get stdout and stderr
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    println!("ripgrep stdout: {}", stdout);
    println!("ripgrep stderr: {}", stderr);
    
    // Parse stats from ripgrep output
    let mut stats = String::new();
    let mut results = String::new();
    
    for line in stdout.lines() {
        if line.starts_with("scanned_files:") || 
           line.starts_with("files_with_matches:") || 
           line.starts_with("total_matches:") || 
           line.starts_with("duration_ms:") {
            stats.push_str(line);
            stats.push('\n');
        } else {
            results.push_str(line);
            results.push('\n');
        }
    }
    
    // Format the final output
    let mut output = String::new();
    output.push_str("<stats>\n");
    output.push_str(&stats);
    output.push_str("</stats>\n");
    output.push_str(&results);
    
    println!("Final output: {}", output);
    
    Ok(output)
}

#[command]
fn open_folder_dialog() -> Option<String> {
    // Purpose: Opens a native folder picker dialog and returns the selected path.
    // Used by: Frontend to let user select a folder to search in.
    FileDialog::new()
        .set_directory(".")
        .pick_folder()
        .map(|p| p.display().to_string())
}

#[command]
fn read_file(path: String, line_number: Option<usize>) -> Result<String, String> {
    // Purpose: Reads a file and optionally provides a preview around a specific line.
    // Used by: Frontend to preview file content for a search result.
    // Special: Logs debug info to file for troubleshooting.
    log_debug(&format!("=== Starting file preview for: {} at line: {:?}", path, line_number));
    
    // Validate path
    if path.trim().is_empty() {
        log_debug("Error: Empty file path");
        return Err("File path cannot be empty".to_string());
    }

    // Get the current working directory from the last search
    let current_dir = match std::env::var("LAST_SEARCH_DIR") {
        Ok(dir) => {
            log_debug(&format!("Using last search directory: {}", dir));
            std::path::PathBuf::from(dir)
        },
        Err(_) => {
            match std::env::current_dir() {
                Ok(dir) => {
                    log_debug(&format!("Using current directory: {}", dir.display()));
                    dir
                },
                Err(e) => {
                    log_debug(&format!("Failed to get current directory: {}", e));
                    return Err(format!("Failed to get current directory: {}", e));
                }
            }
        }
    };

    // Convert path to absolute
    let absolute_path = if path.starts_with("./") {
        let path = current_dir.join(&path[2..]);
        log_debug(&format!("Converted relative path '{}' to: {}", path.to_string_lossy(), path.display()));
        path
    } else if !std::path::Path::new(&path).is_absolute() {
        let path = current_dir.join(&path);
        log_debug(&format!("Converted relative path '{}' to: {}", path.to_string_lossy(), path.display()));
        path
    } else {
        let path = std::path::PathBuf::from(&path);
        log_debug(&format!("Using absolute path: {}", path.display()));
        path
    };

    log_debug(&format!("Attempting to read file at absolute path: {}", absolute_path.display()));

    // Check if file exists
    if !absolute_path.exists() {
        log_debug(&format!("Error: File does not exist at path: {}", absolute_path.display()));
        return Err(format!("File does not exist: {}", absolute_path.display()));
    }

    // Check if it's a file
    if !absolute_path.is_file() {
        log_debug(&format!("Error: Path is not a file: {}", absolute_path.display()));
        return Err(format!("Path is not a file: {}", absolute_path.display()));
    }

    // Try to read the file
    let mut file = match fs::File::open(&absolute_path) {
        Ok(f) => {
            log_debug(&format!("Successfully opened file: {}", absolute_path.display()));
            f
        },
        Err(e) => {
            log_debug(&format!("Failed to open file: {}", e));
            return Err(format!("Failed to open file: {}", e));
        }
    };

    // Read file content
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => log_debug(&format!("Successfully read file content ({} bytes)", content.len())),
        Err(e) => {
            log_debug(&format!("Failed to read file content: {}", e));
            return Err(format!("Failed to read file: {}", e));
        }
    }

    // Check if the content looks like binary data
    if content.len() > 1_000_000 { // 1MB limit
        log_debug("Error: File is too large to preview (>1MB)");
        return Err("File is too large to preview".to_string());
    }

    // Check if the content contains non-text characters
    if content.chars().any(|c| !c.is_ascii() && !c.is_whitespace()) {
        log_debug("Error: File contains non-text characters");
        return Err("File appears to be binary or contains non-text characters".to_string());
    }

    // If line number is provided, try to center the preview around that line
    if let Some(line_num) = line_number {
        let lines: Vec<&str> = content.lines().collect();
        log_debug(&format!("Total lines in file: {}", lines.len()));
        
        if line_num > 0 && line_num <= lines.len() {
            let start = line_num.saturating_sub(10); // Show 10 lines before
            let end = (line_num + 10).min(lines.len()); // Show 10 lines after
            log_debug(&format!("Showing lines from {} to {}", start, end));
            
            // Create a new string with line numbers preserved
            let mut preview_content = String::new();
            for (i, line) in lines[start..end].iter().enumerate() {
                let actual_line_num = start + i + 1; // +1 because line numbers are 1-based
                preview_content.push_str(&format!("{}\n", line));
            }
            
            log_debug(&format!("Created preview with {} lines", end - start));
            return Ok(preview_content);
        } else {
            log_debug(&format!("Line number {} is out of range (1-{})", line_num, lines.len()));
        }
    }

    log_debug(&format!("Successfully read file: {}", absolute_path.display()));
    Ok(content)
}

// Debug logging function
fn log_debug(msg: &str) {
    // Purpose: Writes debug messages to a log file with timestamp.
    // Used by: All backend functions for error and event logging.
    // Special: Ensures logs directory exists, appends to log file.
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");
    
    let log_msg = format!("[{}] {}", timestamp, msg);
    println!("{}", log_msg);
    
    // In development mode, always try to create logs in the project directory first
    #[cfg(debug_assertions)]
    {
        if let Ok(project_dir) = std::env::current_dir() {
            let logs_dir = project_dir.join("logs");
            match fs::create_dir_all(&logs_dir) {
                Ok(_) => println!("Ensured logs directory exists at: {}", logs_dir.display()),
                Err(e) => println!("Failed to create logs directory: {}", e),
            }
            
            let log_file = logs_dir.join("search_tool_debug.log");
            match OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file)
            {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "{}", log_msg) {
                        println!("Failed to write to log file: {}", e);
                    } else {
                        println!("Log written to: {}", log_file.display());
                    }
                },
                Err(e) => println!("Failed to open log file: {}", e),
            }
            return;
        }
    }
    
    // Try multiple locations for logs
    let possible_locations = [
        // 1. Next to the executable
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf())),
        // 2. Current working directory
        std::env::current_dir().ok(),
        // 3. User's home directory
        dirs::home_dir(),
        // 4. Temp directory as last resort
        Some(temp_dir()),
    ];

    let mut logged = false;
    for location in possible_locations.iter().flatten() {
        let logs_dir = location.join("logs");
        
        // Create logs directory if it doesn't exist
        if !logs_dir.exists() {
            if let Err(e) = fs::create_dir_all(&logs_dir) {
                println!("Failed to create logs directory at {}: {}", logs_dir.display(), e);
                continue;
            }
        }
        
        let log_file = logs_dir.join("search_tool_debug.log");
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
        {
            if let Err(e) = writeln!(file, "{}", log_msg) {
                println!("Failed to write to log file at {}: {}", log_file.display(), e);
                continue;
            }
            logged = true;
            println!("Log written to: {}", log_file.display());
            break;
        }
    }

    if !logged {
        println!("CRITICAL: Failed to write to any log location!");
        // Write to a temporary file as last resort
        if let Ok(mut temp_file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(temp_dir().join("search_tool_debug.log"))
        {
            let _ = writeln!(temp_file, "{}", log_msg);
            println!("Fallback log written to temp directory");
        }
    }
}
