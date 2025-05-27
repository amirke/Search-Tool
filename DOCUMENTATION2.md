# Search Tool Documentation

## Project Structure

The project follows a modular architecture with clear separation of concerns:

```
src/
├── components/         # Reusable UI components
├── services/          # Business logic and API calls
├── types/            # TypeScript type definitions
├── utils/            # Utility functions
└── routes/           # SvelteKit page components
```

### Module Roles

#### Components (`src/components/`)
- **SearchForm.svelte**: Handles user input for search queries and folder selection
  - Provides a clean interface for entering search terms
  - Manages folder selection through the native file dialog
  - Emits events for search actions

- **SearchResults.svelte**: Displays search results in a structured format
  - Shows file names with match counts
  - Displays line numbers and content for each match
  - Handles path display and truncation

#### Services (`src/services/`)
- **searchService.ts**: Manages all Tauri API interactions
  - Handles folder browsing through native dialogs
  - Manages search operations via ripgrep
  - Provides port status checking functionality
  - Error handling and result parsing

#### Types (`src/types/`)
- **search.ts**: TypeScript interfaces for type safety
  - `SearchLine`: Represents a single line match
  - `SearchFile`: Groups matches by file
  - `SearchParams`: Search query parameters
  - `SearchResult`: Complete search result structure

#### Utils (`src/utils/`)
- **pathUtils.ts**: Path manipulation utilities
  - Converts absolute paths to relative paths
  - Handles path display formatting
  - Normalizes path separators

- **resultParser.ts**: Search result parsing
  - Parses raw ripgrep output
  - Structures results for display
  - Handles error cases

#### Routes (`src/routes/`)
- **+page.svelte**: Main application page
  - Orchestrates components and services
  - Manages application state
  - Handles error display

## Features

- 🔍 Fast text search using ripgrep
- 📂 Native file dialog integration
- 📝 Line number display
- 📊 Match count per file
- 🎨 Modern, clean interface
- 🔄 Real-time search results
- 📱 Responsive design

## Technical Details

### Frontend
- Built with SvelteKit for a modern, reactive UI
- TypeScript for type safety
- Modular component architecture
- Responsive design with CSS

### Backend
- Rust for high-performance search operations
- Tauri for native system integration
- ripgrep for fast text search
- Native file system access

## Development Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Run in development mode:
   ```bash
   npm run tauri dev
   ```

## Debugging

### Frontend Debugging
- Use browser DevTools (F12)
- Console logging for state tracking
- Svelte DevTools for component inspection

### Backend Debugging
- Rust console output
- Tauri debug logs
- ripgrep verbose mode

## Creating Releases

### Development Build
```bash
npm run tauri dev
```
Output: `src-tauri/target/debug/search-tool.exe`

### Production Build
```bash
npm run tauri build
```
Output: 
- Executable: `src-tauri/target/release/search-tool.exe`
- Installer: `src-tauri/target/release/bundle/msi/search-tool_*.msi`
