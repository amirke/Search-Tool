# Search Tool - Version 1 Documentation

## Overview
The application is a desktop search tool built with Tauri, combining a Svelte frontend with a Rust backend. It uses ripgrep (`rg`) for fast text searching across files.

## Architecture

### Frontend (`src/routes/+page.svelte`)
The frontend is built with Svelte and provides the user interface and result display.

#### Key Components:

1. **State Management**
```typescript
let query = '';           // Search term
let path = '.';           // Search directory
let results = '';         // Raw search results
let error = '';           // Error messages
```

2. **Core Functions**

a) **Folder Selection**
```typescript
async function browseFolder() {
    // Opens native file dialog via Tauri
    // Updates path state with selected folder
}
```

b) **Search Execution**
```typescript
async function search() {
    // Validates inputs
    // Calls Rust backend via Tauri invoke
    // Handles results or errors
}
```

c) **Path Handling**
```typescript
function getRelativePath(fullPath: string) {
    // Converts absolute paths to relative paths
    // Handles Windows/Unix path differences
}

function getDisplayPath(fullPath: string) {
    // Truncates long paths with ellipsis
    // Limits display to 50 characters
}
```

d) **Result Parsing**
```typescript
function parseResults(text: string) {
    // Parses XML-like formatted results
    // Groups results by file
    // Structures data for display
}
```

3. **UI Components**
- Search input field
- Path input field with browse button
- Results display with:
  - File headers showing match counts
  - Line numbers and content
  - Horizontal scrolling for long lines

### Backend (`src-tauri/src/main.rs`)
The Rust backend handles file system operations and search execution.

#### Key Components:

1. **Main Application Setup**
```rust
fn main() {
    // Initializes Tauri
    // Registers commands for frontend access
}
```

2. **Search Implementation**
```rust
#[command]
fn search_text(query: String, path: String) -> Result<String, String> {
    // Input validation
    // Path existence check
    // ripgrep execution
    // Result formatting
}
```

3. **File Dialog**
```rust
#[command]
fn open_folder_dialog() -> Option<String> {
    // Opens native folder selection dialog
    // Returns selected path
}
```

## Data Flow

1. **Search Process**:
   ```
   User Input → Frontend Validation → Tauri Invoke → Rust Backend → ripgrep → Results → Frontend Display
   ```

2. **Result Format**:
   ```
   <line file="path/to/file" num="123">content</line>
   ```

## Error Handling

### Frontend
- Input validation
- Search error display
- Result parsing error handling
- Path normalization errors

### Backend
- Path existence validation
- ripgrep execution errors
- Result formatting errors

## Styling
- Modern, clean interface
- Responsive design
- Monospace font for code
- Clear error messages
- Horizontal scrolling for long lines

## Key Features
1. Fast text search using ripgrep
2. Native file dialog integration
3. Relative path display
4. Line number display
5. Match count per file
6. Error handling and display
7. Path truncation for long paths
8. Binary content filtering
9. Split view with file preview
10. Highlighted search matches in preview

## UI Layout

### Split View
The application window is divided into two main sections:

1. **Search Panel (Left)**
   - Search input and controls
   - Search results list
   - File headers with match counts
   - Line numbers and content preview

2. **File Preview Panel (Right)**
   - Full file content display
   - Syntax highlighting
   - Yellow highlighting for search matches
   - Line numbers
   - Scrollable view
   - Appears when a search result is clicked

### Interaction Flow
1. User performs search in left panel
2. Results are displayed in left panel
3. Clicking a result:
   - Opens the file in right panel
   - Highlights all matches in yellow
   - Scrolls to the first match
   - Maintains search context

### Preview Features
- Syntax highlighting based on file type
- Yellow background for matched text
- Line numbers
- Horizontal and vertical scrolling
- Maintains search context when switching files
- Responsive layout (adjusts to window size)

## Technical Details

### Frontend Dependencies
- Svelte
- Tauri API
- TypeScript

### Backend Dependencies
- Tauri
- ripgrep
- rfd (Rust File Dialog)

### Search Options
```rust
.arg("--line-number")     // Show line numbers
.arg("--with-filename")   // Always show filename
.arg("--smart-case")      // Case sensitive only if query contains uppercase
.arg("--no-ignore")       // Don't respect .gitignore
.arg("--hidden")          // Search hidden files
```

## Development Setup

1. **Prerequisites**
   - Node.js and npm
   - Rust and Cargo
   - ripgrep installed on your system

2. **Installation**
   ```bash
   # Install dependencies
   npm install
   
   # Run development server
   npm run tauri dev
   ```

3. **Building**
   ```bash
   # Build for production
   npm run tauri build
   ```

## Creating Releases

### Building the Executable
1. **Development Build**
   ```bash
   npm run tauri dev
   ```
   - Creates a development version
   - Includes debug information
   - Faster build time
   - Output: `src-tauri/target/debug/search-tool.exe`

2. **Production Build**
   ```bash
   npm run tauri build
   ```
   - Creates optimized release version
   - Removes debug information
   - Longer build time
   - Output: `src-tauri/target/release/search-tool.exe`
   - Also creates installer: `src-tauri/target/release/bundle/msi/search-tool_*.msi`

### Release Package Contents
- `search-tool.exe`: Standalone executable
- `search-tool_*.msi`: Windows installer
- Required DLLs and dependencies
- Application icons and resources

### Distribution
1. **Standalone Executable**
   - Share the `.exe` file directly
   - Users can run it without installation
   - Portable version

2. **Installer**
   - Share the `.msi` file
   - Users can install it properly
   - Creates start menu entries
   - Enables uninstallation

## Debugging

### Frontend Debugging
1. **Browser DevTools**
   - Press `F12` in the app window
   - View console logs
   - Inspect elements
   - Debug JavaScript/TypeScript

2. **Console Logging**
   ```typescript
   console.log('[Frontend] Message');
   console.error('[Frontend] Error:', error);
   ```

3. **Svelte DevTools**
   - Install Svelte DevTools browser extension
   - Inspect component state
   - Monitor reactivity

### Backend Debugging
1. **Rust Console Output**
   ```rust
   println!("[Backend] Message");
   eprintln!("[Backend] Error: {}", error);
   ```

2. **Debug Build**
   ```bash
   npm run tauri dev
   ```
   - Includes debug information
   - Shows detailed error messages
   - Enables breakpoints

3. **Common Issues**
   - Port conflicts (1420)
   - File permission errors
   - Path resolution problems
   - ripgrep execution failures

### Debugging Tips
1. **Check Logs**
   - Frontend: Browser console
   - Backend: Terminal output
   - System: Event Viewer

2. **Common Problems**
   - Port 1420 in use
   - Missing ripgrep
   - File access permissions
   - Path encoding issues

3. **Performance Issues**
   - Large file handling
   - Search result parsing
   - UI rendering
   - Memory usage

## Troubleshooting

### Common Issues

1. **ripgrep not found**
   - Ensure ripgrep is installed and in your system PATH
   - Windows: Install via `scoop install ripgrep` or `choco install ripgrep`

2. **Search not working**
   - Check console for error messages
   - Verify path exists and is accessible
   - Ensure search query is not empty

3. **UI Issues**
   - Clear browser cache
   - Restart development server
   - Check browser console for errors

4. **Build Issues**
   - Ensure all dependencies are installed
   - Check Rust toolchain is up to date
   - Verify Node.js version compatibility
   - Clear build cache if needed

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License
[Add your license information here] 

## Svelte Code

```
import { onMount } from 'svelte';

let statusMessage = '';

onMount(() => {
  // This message will show in the browser console and can be shown in the UI
  statusMessage = 'Checking port 1420 availability...';
  console.log('[Frontend] ' + statusMessage);
});
```

{#if statusMessage}
  <div class="status">{statusMessage}</div>
{/if}
