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

use once_cell::sync::Lazy;  //מאפשר להגדיר משתנים גלובליים שנטענים רק פעם אחת כשצריך
use std::collections::HashMap; // מבנה נתונים של מפתח-ערך, משמש לשמירת LineIndex לכל path
use std::sync::Mutex;   // מאפשר לגשת למשתנה משותף בבטחה מתהליכים שונים


use std::process::Command;         // To run shell commands (ripgrep)
use tauri::command;                // Attribute to expose functions to JS
use rfd::FileDialog;              // For native file dialogs
use std::fs;
use std::os::windows::process::CommandExt;  // For creation_flags
use std::env::temp_dir;  // Add this for temporary directory

// use std::fs::File;
// use std::io::{BufRead, BufReader};

// use std::io::Read;
use std::io::Write;
// use encoding_rs::WINDOWS_1252;

mod memmap_line_reader;
use memmap_line_reader::LineIndex;

// Embed the ripgrep binary
const RG_BINARY: &[u8] = include_bytes!("../bin/rg.exe");

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

#[derive(serde::Serialize)]
struct FileChunk {
    lines: Vec<String>,
    offset: usize,
    has_more: bool,
}


#[derive(serde::Serialize)]
struct ChunkResponse {
    lines: Vec<String>,
    offset: usize,
    has_more: bool,
}

// A global cache that maps absolute file paths to LineIndex
static INDEX_CACHE: Lazy<Mutex<HashMap<String, LineIndex>>> = Lazy::new(|| Mutex::new(HashMap::new()));

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
            tauri::generate_handler![search_text, open_folder_dialog, read_file_mmap_chunk, get_about_info] //what are these handlers?
                // search_text is the function that is called when the user clicks the search button
                // open_folder_dialog is the function that is called when the user clicks the open folder button
                // read_file is the function that is called when the user clicks the read file butto. its not a button. its when pressing on the results
                // get_about_info is the function that is called when the user clicks the about button
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn read_file_mmap_chunk(path: String, offset: usize, count: usize) -> Result<ChunkResponse, String> {
    log_debug(&format!("Reading file: {}", path));
    log_debug(&format!("Offset: {}", offset));
    log_debug(&format!("Count: {}", count));

    // Try getting the directory from env or fallback to current dir
    let current_dir = std::env::var("LAST_SEARCH_DIR")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    log_debug(&format!("Current directory: {}", current_dir.display()));

    // Resolve absolute path safely
    let abs_path = if path.starts_with("./") {
        current_dir.join(&path[2..])
    } else if !std::path::Path::new(&path).is_absolute() {
        current_dir.join(&path)
    } else {
        std::path::PathBuf::from(&path)
    };

    log_debug(&format!("Absolute path: {}", abs_path.display()));

    // Check file exists
    if !abs_path.exists() {
        return Err(format!("File does not exist: {}", abs_path.display()));
    }

    let abs_path_str = abs_path.to_string_lossy().to_string();

    let mut cache = INDEX_CACHE.lock().unwrap();

    if !cache.contains_key(&abs_path_str) {
        let index = LineIndex::new(&abs_path_str)
            .map_err(|e| format!("Failed to index file: {}", e))?;
        cache.insert(abs_path_str.clone(), index);
    }

    let index = cache.get(&abs_path_str).unwrap();
    let total_lines = index.line_count();
    let lines = index.get_lines(offset, count);
    let next_offset = offset + lines.len();

    Ok(ChunkResponse {
        lines,
        offset: next_offset,
        has_more: next_offset < total_lines,
    })
}






// -----------------------------------------------------------
// This function is exposed to the frontend via Tauri's `invoke`
// It runs `ripgrep` (rg) to search for `query` in `path`
// -----------------------------------------------------------
#[command]  // Expose this function to the frontend via Tauri
fn search_text(
        _app: tauri::AppHandle,
        query: String,
        path: String,
        file_filter: Option<String>,
        case_sensitive: bool,
        whole_phrase: bool,
        whole_words: bool,
    ) -> Result<String, String> {
    
    log_debug("=== Starting new search ===");
    log_debug(&format!("Query: '{}'", query));
    log_debug(&format!("Path: '{}'", path));
    log_debug(&format!("Case sensitive: '{}'", case_sensitive));
    log_debug(&format!("Whole phrase: '{}'", whole_phrase));
    log_debug(&format!("Whole words: '{}'", whole_words));
    if let Some(ref filter) = file_filter {
        log_debug(&format!("File filter: '{}'", filter));
    }
    std::env::set_var("LAST_SEARCH_DIR", &path);
    if query.trim().is_empty() {
        log_debug("Error: Empty query");
        return Ok("Search query cannot be empty".to_string());
    }
    if path.trim().is_empty() {
        log_debug("Error: Empty path");
        return Ok("Search path cannot be empty".to_string());
    }
    if !std::path::Path::new(&path).exists() {
        log_debug(&format!("Error: Path does not exist: {}", path));
        return Ok(format!("Path does not exist: {}", path));
    }

    let temp_dir = temp_dir();
    // 🛡️ One-liner: wait if rg.exe exists and is locked
    if temp_dir.join("rg.exe").exists() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    let rg_path = temp_dir.join("rg.exe");

    log_debug("Extracting ripgrep binary");
    match fs::write(&rg_path, RG_BINARY) {
        Ok(_) => log_debug("Successfully extracted ripgrep binary"),
        Err(e) => {
            log_debug(&format!("Failed to extract ripgrep: {}", e));
            return Ok(format!("Failed to extract ripgrep: {}", e));
        }
    }
    let mut cmd = Command::new(&rg_path);
    cmd.current_dir(&path)
        .arg("--line-number")
        .arg("--with-filename")
        .arg("--no-ignore")
        .arg("--hidden")
        .arg("--text")
        .arg("--stats");

    // Handle case sensitivity
    if case_sensitive {
        cmd.arg("--case-sensitive");
    } else {
        cmd.arg("--ignore-case");
    }
    
    // Handle whole phrase (literal string match)
    if whole_phrase {
        cmd.arg("--fixed-strings"); // ensures query is interpreted literally
    }

    // Handle whole words (word regexp)
    if whole_words {
        cmd.arg("--word-regexp");
    }

    // Handle file filter (glob pattern)
    if let Some(filter) = file_filter {
        cmd.arg("--glob").arg(filter);
    }

    //
    cmd.arg(&query).arg(".")
        .creation_flags(0x08000000);

    // Run the command and get the output
    let output = cmd.output();
    let _ = fs::remove_file(&rg_path);
    log_debug("Cleaned up temporary ripgrep binary");

    // Match the output of the ripgrep command to get the stats and the results
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
            
        
// Format the results with HTML-like tags for styling
// Create a string with this format
// <line file="file_name" num="line_number">content</line>            

            let formatted = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let first_colon = line.find(':').unwrap_or(0); //find the first colon and second colon to find the file name and line number next to each other
                let second_colon = line[first_colon + 1..]
                    .find(':')
                    .map(|i| i + first_colon + 1)
                    .unwrap_or(0);
                if first_colon > 0 && second_colon > first_colon {
                    let file_name = line[..first_colon].trim().replace('\\', "/");
                    let line_num = line[first_colon + 1..second_colon].trim();
                    let content = line[second_colon + 1..].trim();
        
                    if content.len() > 1000 || content.chars().any(|c| !c.is_ascii() && !c.is_whitespace()) {
                        return String::new();
                    }
                    let escaped_content = content
                        .replace("&", "&amp;")
                        .replace("<", "&lt;")
                        .replace(">", "&gt;")
                        .replace("\"", "&quot;")
                        .replace("'", "&apos;");
                    format!("<line file=\"{}\" num=\"{}\">{}</line>", file_name, line_num, escaped_content)                } else {
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
            
            // Create a response struct with the formatted HTML and stats
            let response = SearchResponse {
                html: formatted,    // the formatted HTML is from the ripgrep output
                stats,              // the stats are from the ripgrep output
            };
            
            // Convert the response to a JSON string
            let json = serde_json::to_string(&response).unwrap();  // convert the response to a JSON string by using serde_json which is a library for serializing and deserializing JSON
            // Delete it since its too long to display
            // log_debug(&format!("Final JSON result: {}", json));
            Ok(json)
            }
        Err(e) => {
            log_debug(&format!("Failed to run ripgrep: {}", e));
            log_debug("=== Search failed ===");
            Err(format!("Failed to run ripgrep: {}", e))
        }
    }
}

// This command is exposed to the Tauri frontend.
// It reads the content of a file, trying UTF-8 and then Windows-1252 encoding.
#[command]
fn open_folder_dialog() -> Option<String> {
    FileDialog::new()
        .set_directory(".")
        .pick_folder()
        .map(|p| p.display().to_string())
}



// this is the function that is called when the user clicks the read file button
// it is called when the user clicks on the results actuallu
// Comment all read_file function because it is not needed at the current stage
/* 
#[command]
fn read_file(path: String, line_number: Option<usize>) -> Result<String, String> {
    log_debug(&format!("=== Starting file preview for: {} at line: {:?}", path, line_number));
    
    // Validate path - if the path is empty, return an error
    if path.trim().is_empty() {
        log_debug("Error: Empty file path");
        return Ok("File path cannot be empty".to_string());
    }

 // No need at the current stage
 //   // Remove any line number suffix from the path (e.g., "file.txt:123" -> "file.txt")
 //   let clean_path = if let Some(pos) = path.rfind(':') {  
 //       &path[..pos]
 //   } else {
 //       &path
 //   };
    let clean_path = path;

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
                    return Ok(format!("Failed to get current directory: {}", e));
                }
            }
        }
    };

// check if works/
    //     let current_dir = std::env::var("LAST_SEARCH_DIR")
    // .ok()
    // .map(std::path::PathBuf::from)
    // .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    //    let absolute_path = if clean_path.starts_with("./") {
    //    current_dir.join(&clean_path[2..])
    // } else if !std::path::Path::new(clean_path).is_absolute() {
    //    current_dir.join(clean_path)
    // } else {
    //    std::path::PathBuf::from(clean_path)
    // };

    // if !absolute_path.exists() || !absolute_path.is_file() {
    //     return Ok(format!("Invalid file path: {}", absolute_path.display()));
    // }
//////////////////////////////////////
    // Convert file path to absolute path
    let absolute_path = if clean_path.starts_with("./") {   // if the path starts with ./, it is a relative path
        let path = current_dir.join(&clean_path[2..]);  // remove the ./ from the path
        log_debug(&format!("Converted 1relative path '{}' to: {}", clean_path, path.display()));
        path
    } else if !std::path::Path::new(clean_path).is_absolute() {  // if the path is not absolute, it is a relative path
        let path = current_dir.join(clean_path);
        log_debug(&format!("Converted 2relative path '{}' to: {}", clean_path, path.display()));
        path
    } else {
        // Already absolute path
        let path = std::path::PathBuf::from(clean_path);
        log_debug(&format!("Using absolute path: {}", path.display()));
        path
    };

    log_debug(&format!("Attempting to read file at absolute path: {}", absolute_path.display()));

    // Check if file exists
    if !absolute_path.exists() {
        log_debug(&format!("Error: File does not exist at path: {}", absolute_path.display()));
        return Ok(format!("File does not exist: {}", absolute_path.display()));
    }

    // Check if it's a file
    if !absolute_path.is_file() {
        log_debug(&format!("Error: Path is not a file: {}", absolute_path.display()));
        return Ok(format!("Path is not a file: {}", absolute_path.display()));
    }
///////////////////////////////////////////////////////////////////////
/// 
    // Try to read the file
    let mut file = match fs::File::open(&absolute_path) {
        Ok(f) => {
            log_debug(&format!("Successfully opened file: {}", absolute_path.display()));
            f
        },
        Err(e) => {
            log_debug(&format!("Failed to open file: {}", e));
            return Ok(format!("Failed to open file: {}", e));
        }
    };

    // Try to read the file as bytes
    let mut buf = Vec::new();
    match file.read_to_end(&mut buf) {
        Ok(_) => log_debug(&format!("Successfully read file content ({} bytes)", buf.len())),
        Err(e) => {
            log_debug(&format!("Failed to read file content: {}", e));
            return Ok(format!("Failed to read file: {}", e));
        }
    }

    // Removing the file size limit
    // if buf.len() > 1_000_000 { // 1MB limit
    //     log_debug("Error: File is too large to preview (>1MB)");
    //     return Err("File is too large to preview".to_string());
    // }
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
*/
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
