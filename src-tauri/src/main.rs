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
use std::io::Write;
use std::env::temp_dir;  // Add this for temporary directory
use encoding_rs::WINDOWS_1252;

// Embed the ripgrep binary
const RG_BINARY: &[u8] = include_bytes!("../bin/rg.exe");

// ----------------------
// Main application entry
// ----------------------
fn main() {
    // Set up logs directory and env var as early as possible
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(app_dir) = exe_path.parent() {
            let logs_dir = app_dir.join("logs");
            let _ = std::fs::create_dir_all(&logs_dir);
            std::env::set_var("LOGS_DIR", logs_dir.to_str().unwrap_or(""));
        }
    }
    
    log_debug("The new search tool in new face 2025 by KV labs");

    #[cfg(target_os = "windows")]
    install_webview2_if_needed();

    println!("Tauri backend starting...");
    tauri::Builder::default()
        .invoke_handler(
            tauri::generate_handler![search_text, open_folder_dialog, read_file, get_about_info]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// -----------------------------------------------------------
// This struct is used to store the search stats
// -----------------------------------------------------------
#[derive(serde::Serialize)]
struct SearchStats {
    total_matches: usize,
    matched_lines: usize,
    files_searched: usize,
    search_time_ms: f64,
    total_time_ms: f64,
}

// -----------------------------------------------------------
// This function is exposed to the frontend via Tauri's `invoke`
// It runs `ripgrep` (rg) to search for `query` in `path`
// -----------------------------------------------------------
#[command]  // Expose this function to the frontend via Tauri
fn search_text(_app: tauri::AppHandle, query: String, path: String, file_filter: Option<String>) -> Result<String, String> {
    log_debug("=== Starting new search ===");
    log_debug(&format!("Query: '{}'", query));
    log_debug(&format!("Path: '{}'", path));
    if let Some(ref filter) = file_filter {
        log_debug(&format!("File filter: '{}'", filter));
    }
    std::env::set_var("LAST_SEARCH_DIR", &path);
    if query.trim().is_empty() {
        log_debug("Error: Empty query");
        return Err("Search query cannot be empty".to_string());
    }
    if path.trim().is_empty() {
        log_debug("Error: Empty path");
        return Err("Search path cannot be empty".to_string());
    }
    if !std::path::Path::new(&path).exists() {
        log_debug(&format!("Error: Path does not exist: {}", path));
        return Err(format!("Path does not exist: {}", path));
    }
    let temp_dir = temp_dir();
    let rg_path = temp_dir.join("rg.exe");
    log_debug("Extracting ripgrep binary");
    match fs::write(&rg_path, RG_BINARY) {
        Ok(_) => log_debug("Successfully extracted ripgrep binary"),
        Err(e) => {
            log_debug(&format!("Failed to extract ripgrep: {}", e));
            return Err(format!("Failed to extract ripgrep: {}", e));
        }
    }
    let mut cmd = Command::new(&rg_path);
    cmd.current_dir(&path)
        .arg("--line-number")
        .arg("--with-filename")
        .arg("--smart-case")
        .arg("--no-ignore")
        .arg("--hidden")
        .arg("--text")
        .arg("--fixed-strings")
        .arg("--stats");
    if let Some(filter) = file_filter {
        cmd.arg("--glob").arg(filter);
    }
    cmd.arg(&query).arg(".")
        .creation_flags(0x08000000);
    let output = cmd.output();
    let _ = fs::remove_file(&rg_path);
    log_debug("Cleaned up temporary ripgrep binary");

    match output {
        Ok(output) => {
            log_debug(&format!("ripgrep exit status: {:?}", output.status));
            
            // Get the output as strings
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // Log any stderr output if present
            if !stderr.is_empty() {
                log_debug(&format!("ripgrep stderr: {}", stderr));
            }

            let mut stats = SearchStats {
                total_matches: 0,
                matched_lines: 0,
                files_searched: 0,
                search_time_ms: 0.0,
                total_time_ms: 0.0,
            };
            
            for line in stdout.lines() {
                if let Some(num) = line.split_whitespace().next() {
                    if line.contains("matches") && !line.contains("matched lines") {
                        stats.total_matches = num.parse().unwrap_or(0);
                    } else if line.contains("matched lines") {
                        stats.matched_lines = num.parse().unwrap_or(0);
                    } else if line.contains("files searched") {
                        stats.files_searched = num.parse().unwrap_or(0);
                    } else if line.contains("seconds spent searching") {
                        if let Some(num) = line.split_whitespace().next() {
                            if let Ok(sec) = num.trim().parse::<f64>() {
                                stats.search_time_ms = sec * 1000.0; // Convert to milliseconds
                            }
                        }
                    } else if line.contains(" seconds") && !line.contains("spent searching") {
                        if let Some(num) = line.split_whitespace().next() {
                            if let Ok(sec) = num.trim().parse::<f64>() {
                                stats.total_time_ms = sec * 1000.0; // Convert to milliseconds
                            }
                        }
                    }                            
                }
            }
            
            log_debug("=== Ripgrep stats ===");
            log_debug(&format!("files contained matches: {}", stats.total_matches));
            log_debug(&format!("Matched lines: {}", stats.matched_lines));
            log_debug(&format!("Files searched: {}", stats.files_searched));
            log_debug(&format!("Search time (ms): {:.3}", stats.search_time_ms));
            log_debug(&format!("Total time (ms): {:.3}", stats.total_time_ms));

            // Check if we got any results
            if stdout.trim().is_empty() {
                log_debug("No matches found for the search query");
                return Ok("".to_string()); // Return empty string instead of error
            }
            
            log_debug(&format!("Found {} result lines", stdout.lines().count()));
        
        // Format the results with HTML-like tags for styling
            let formatted = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let last_colon = line.rfind(':').unwrap_or(0);
                let second_last_colon = line[..last_colon].rfind(':').unwrap_or(0);
                if last_colon > 0 && second_last_colon > 0 {
                    let file_name = line[..second_last_colon].trim().replace('\\', "/");
                    let line_num = line[second_last_colon+1..last_colon].trim();
                    let content = line[last_colon+1..].trim();
                    if content.len() > 1000 || content.chars().any(|c| !c.is_ascii() && !c.is_whitespace()) {
                        return String::new();
                    }
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
                .collect::<Vec<_>>()
            .join("\n");
            log_debug("=== Search completed successfully ===");

            #[derive(serde::Serialize)]
            struct SearchResponse {
                html: String,
                stats: SearchStats,
            }
            
            let response = SearchResponse {
                html: formatted,
                stats,
            };
            
            let json = serde_json::to_string(&response).unwrap();
            log_debug(&format!("Final JSON result: {}", json));
            Ok(json)
        }
        Err(e) => {
            log_debug(&format!("Failed to run ripgrep: {}", e));
            log_debug("=== Search failed ===");
            Err(format!("Failed to run ripgrep: {}", e))
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
fn read_file(path: String, line_number: Option<usize>) -> Result<String, String> {
    log_debug(&format!("=== Starting file preview for: {} at line: {:?}", path, line_number));
    
    // Validate path
    if path.trim().is_empty() {
        log_debug("Error: Empty file path");
        return Err("File path cannot be empty".to_string());
    }

    // Remove any line number suffix from the path (e.g., "file.txt:123" -> "file.txt")
    let clean_path = if let Some(pos) = path.rfind(':') {
        &path[..pos]
    } else {
        &path
    };

    // Get the current working directory from the last search
    let current_dir = match std::env::var("LAST_SEARCH_DIR") {
        Ok(dir) => {
            std::path::PathBuf::from(dir)
        },
        Err(_) => {
            match std::env::current_dir() {
                Ok(dir) => {
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
    let absolute_path = if clean_path.starts_with("./") {
        let path = current_dir.join(&clean_path[2..]);
        log_debug(&format!("Converted relative path '{}' to: {}", clean_path, path.display()));
        path
    } else if !std::path::Path::new(clean_path).is_absolute() {
        let path = current_dir.join(clean_path);
        log_debug(&format!("Converted relative path '{}' to: {}", clean_path, path.display()));
        path
    } else {
        let path = std::path::PathBuf::from(clean_path);
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

    // Try to read the file as bytes
    let mut buf = Vec::new();
    match file.read_to_end(&mut buf) {
        Ok(_) => log_debug(&format!("Successfully read file content ({} bytes)", buf.len())),
        Err(e) => {
            log_debug(&format!("Failed to read file content: {}", e));
            return Err(format!("Failed to read file: {}", e));
        }
    }
    if buf.len() > 1_000_000 { // 1MB limit
        log_debug("Error: File is too large to preview (>1MB)");
        return Err("File is too large to preview".to_string());
    }
    // Try UTF-8 first
    if let Ok(content) = String::from_utf8(buf.clone()) {
        log_debug(&format!("Successfully decoded as UTF-8 ({} bytes)", content.len()));
        return Ok(content);
    }
    // Try Windows-1252
    let (cow, _, had_errors) = WINDOWS_1252.decode(&buf);
    if !had_errors {
        let content = cow.into_owned();
        log_debug(&format!("Successfully decoded as Windows-1252 ({} bytes)", content.len()));
        return Ok(content);
    }
    log_debug("File encoding not supported or file is binary");
    Err("File encoding not supported or file is binary".to_string())
}

#[tauri::command]
fn get_about_info(app_handle: tauri::AppHandle) -> Result<(String, String), String> {
    // Embed about.txt at compile time
    let about_text = include_str!("../about.txt").to_string();
    let version = app_handle.package_info().version.to_string();
    Ok((about_text, version))
}

fn log_debug(msg: &str) {
    if let Some(logs_dir) = std::env::var("LOGS_DIR").ok() {
        let log_file = std::path::Path::new(&logs_dir).join("search_tool.log");
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let log_entry = format!("[{}] {}\n", timestamp, msg);
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file) 
        {
            let _ = file.write_all(log_entry.as_bytes());
            let _ = file.flush(); // Ensure log is written immediately
        }
    }
}

/// Check if Microsoft Edge WebView2 Runtime is installed.
/// This function checks:
/// 1. System-wide registry (HKLM) for known product GUIDs
/// 2. User-wide registry (HKCU)
/// 3. File system paths for WebView2 EXE in standard install folders
/// 4. Recursively inside version subfolders
///
/// If none of the checks detect WebView2, the user is prompted to install it manually.
#[cfg(target_os = "windows")]
fn install_webview2_if_needed() {
    use std::fs;
    use std::path::Path;
    use winreg::enums::*;
    use winreg::RegKey;

    log_debug("Checking if WebView2 Runtime is installed...");
    let mut found = false;

    // -- REGISTRY CHECK (HKLM) --
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let system_guids = [
        "{F1E7E2A0-DA5B-4E5B-9B1A-1A1A1A1A1A1A}",
        "{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}",
    ];
    for guid in &system_guids {
        let reg_path = format!("SOFTWARE\\Microsoft\\EdgeUpdate\\Clients\\{}", guid);
        if let Ok(key) = hklm.open_subkey(&reg_path) {
            if let Ok(version) = key.get_value::<String, _>("pv") {
                log_debug(&format!("✅ WebView2 found in registry (HKLM): GUID {}, version {}", guid, version));
                found = true;
                break;
            }
        }
    }

    // -- REGISTRY CHECK (HKCU) --
    if !found {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let user_path = "SOFTWARE\\Microsoft\\EdgeUpdate\\Clients\\{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}";
        if let Ok(key) = hkcu.open_subkey(user_path) {
            if let Ok(version) = key.get_value::<String, _>("pv") {
                log_debug(&format!("✅ WebView2 found in registry (HKCU): version {}", version));
                found = true;
            }
        }
    }

    // -- FILE SYSTEM CHECK --
    let possible_dirs = [
        "C:\\Program Files (x86)\\Microsoft\\EdgeWebView\\Application",
        "C:\\Program Files\\Microsoft\\EdgeWebView\\Application",
    ];
    for base in &possible_dirs {
        let base_path = Path::new(base);
        if base_path.is_dir() {
            // Direct match: C:\...\Application\msedgewebview2.exe
            let direct = base_path.join("msedgewebview2.exe");
            if direct.exists() {
                log_debug(&format!("✅ WebView2 found at: {}", direct.display()));
                found = true;
                break;
            }

            // Check subfolders: C:\...\Application\{version}\msedgewebview2.exe
            if let Ok(entries) = fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let sub = entry.path();
                    if sub.is_dir() {
                        let exe = sub.join("msedgewebview2.exe");
                        if exe.exists() {
                            log_debug(&format!("✅ WebView2 found in subfolder: {}", exe.display()));
                            found = true;
                            break;
                        }
                    }
                }
            }
        }
        if found {
            break;
        }
    }

    // -- NOT FOUND: Prompt user to install --
    if !found {
        log_debug("❌ WebView2 Runtime not found.");
        log_debug("➡️ Please install it from:");
        log_debug("   https://developer.microsoft.com/en-us/microsoft-edge/webview2/");
    }
}
