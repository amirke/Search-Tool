// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::process::Command;
use std::net::TcpListener;
use std::time::Duration;
use std::thread;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn is_port_in_use(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_err()
}

fn kill_process_on_port(port: u16) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        println!("[Backend] Checking for processes using port {}...", port);
        // Get the process ID using netstat
        let output = Command::new("netstat")
            .args(["-ano", "-p", "TCP"])
            .output()
            .map_err(|e| format!("Failed to run netstat: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Find the line containing our port
        for line in output_str.lines() {
            if line.contains(&format!(":{}", port)) && line.contains("LISTENING") {
                // Extract the PID
                if let Some(pid) = line.split_whitespace().last() {
                    if let Ok(pid) = pid.parse::<u32>() {
                        println!("[Backend] Found process {} using port {}, attempting to kill...", pid, port);
                        // Kill the process
                        Command::new("taskkill")
                            .args(["/F", "/PID", &pid.to_string()])
                            .output()
                            .map_err(|e| format!("Failed to kill process: {}", e))?;
                        
                        println!("[Backend] Successfully killed process {}", pid);
                        // Wait a bit for the port to be released
                        thread::sleep(Duration::from_millis(500));
                        return Ok(());
                    }
                }
            }
        }
    }
    
    Err("No process found using the port".to_string())
}

fn ensure_port_available(port: u16) -> Result<(), String> {
    if is_port_in_use(port) {
        println!("[Backend] Port {} is in use, attempting to free it...", port);
        kill_process_on_port(port)?;
        
        // Double check if the port is now available
        if is_port_in_use(port) {
            return Err(format!("Failed to free port {}", port));
        }
        println!("[Backend] Port {} is now available", port);
    } else {
        println!("[Backend] Port {} is available", port);
    }
    Ok(())
}

#[tauri::command]
fn ensure_port_1420_available() -> String {
    match ensure_port_available(1420) {
        Ok(_) => "Port 1420 is now available".to_string(),
        Err(e) => format!("Failed to free port 1420: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Try to ensure port 1420 is available before starting
    if let Err(e) = ensure_port_available(1420) {
        eprintln!("[Backend] Warning: {}", e);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, ensure_port_1420_available])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
