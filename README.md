# Search Tool

A fast, native desktop search tool built with Tauri and Svelte. This tool provides quick text search capabilities across directories using ripgrep.

## Features

- 🔍 Fast text search using ripgrep
- 📂 Native file dialog integration
- 📝 Line number display
- 📊 Match count per file
- 🎨 Modern, clean interface
- 🔄 Real-time search results
- 📱 Responsive design

## Installation

### Prerequisites

- Node.js and npm
- Rust and Cargo
- ripgrep (install via `scoop install ripgrep` or `choco install ripgrep` on Windows)

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/amirke/Search-Tool.git
   cd Search-Tool
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

### Building

To create a production build:
```bash
npm run tauri build
```

The executable will be available in `src-tauri/target/release/`.

## Usage

1. Launch the application
2. Enter your search query
3. Select a directory to search in
4. View results with line numbers and match counts

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Add your license here]

## Version

Initial Release - Simple Fast Search in Directory
