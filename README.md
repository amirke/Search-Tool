# Search Tool

A fast, native desktop search tool built with Tauri and Svelte. This tool provides quick text search capabilities across directories using ripgrep.

## 🏗️ Project Structure

This project is organized into clearly separated directories for better maintainability:

```
SearchTool/
├── Frontend/                 # Svelte/SvelteKit frontend application
│   ├── src/                 # Frontend source code
│   │   ├── components/      # Reusable UI components
│   │   ├── routes/         # SvelteKit page components
│   │   ├── stores/         # Svelte stores for state management
│   │   ├── types/          # TypeScript type definitions
│   │   ├── utils/          # Utility functions
│   │   └── lib/            # Library components
│   ├── static/             # Static assets (icons, images)
│   ├── svelte.config.js    # Svelte configuration
│   ├── vite.config.js      # Vite bundler configuration
│   └── vite.config.ts      # TypeScript Vite configuration
│
├── Backend/                 # Rust/Tauri backend application
│   ├── src/                # Rust source code
│   │   ├── main.rs         # Main application entry point
│   │   ├── lib.rs          # Library functions
│   │   └── memmap_line_reader.rs # Memory-mapped file reader
│   ├── bin/                # External binaries (ripgrep)
│   ├── icons/              # Application icons
│   ├── capabilities/       # Tauri security capabilities
│   ├── wix/               # Windows installer configuration
│   ├── Cargo.toml         # Rust package configuration
│   ├── tauri.conf.json    # Tauri application configuration
│   └── build.ps1          # Build script
│
├── Scripts/                 # Build and automation scripts
│   ├── create-cert.ps1     # Certificate creation script
│   ├── download-rg.ps1     # Ripgrep download script
│   └── download-sdk.ps1    # SDK download script
│
├── Documentation/           # Project documentation
│   ├── README.md           # This file
│   ├── DOCUMENTATION.md    # Detailed technical documentation
│   ├── DOCUMENTATION2.md   # Additional documentation
│   ├── History.md          # Development history
│   ├── arch.md            # Architecture documentation
│   └── nextissues.md      # Future development plans
│
├── icons/                   # Shared application icons
├── package.json            # Node.js package configuration
├── package-lock.json       # Dependency lock file
└── tsconfig.json          # TypeScript configuration
```

## ✨ Features

- 🔍 **Fast text search** using ripgrep for lightning-quick results
- 📂 **Native file dialog** integration for easy folder selection
- 📝 **Line number display** with precise match locations
- 📊 **Match count per file** for quick result overview
- 🎨 **Modern, clean interface** with responsive design
- 🔄 **Real-time search results** as you type
- 📱 **Responsive design** that adapts to window size
- 🖥️ **Split-pane view** with file preview
- 🌟 **Syntax highlighting** in file preview
- 💡 **Smart case-sensitive search** (case-sensitive only when uppercase letters are used)

## 🚀 Quick Start

### Prerequisites

- **Node.js** (v16+) and npm
- **Rust** and Cargo (latest stable)
- **Git** for version control

### Installation

1. **Clone and navigate to the project:**
   ```bash
   git clone <repository-url>
   cd SearchTool
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Run in development mode:**
   ```bash
   npm run tauri:dev
   ```

## 🛠️ Development Commands

### Frontend Development
```bash
npm run dev          # Start frontend development server
npm run build        # Build frontend for production
npm run preview      # Preview production frontend build
npm run check        # Run Svelte type checking
npm run check:watch  # Run Svelte type checking in watch mode
```

### Backend Development
```bash
npm run tauri        # Run Tauri CLI commands
npm run tauri:dev    # Start Tauri development mode
npm run tauri:build  # Build Tauri application
```

### Full Application
```bash
npm start            # Start development build using PowerShell script
npm run build:release # Create production build using PowerShell script
npm run clean        # Clean build artifacts
```

### Signing & Distribution
```bash
npm run tauri:build-signed      # Build and sign MSI installer
npm run tauri:build-signed-nsis # Build and sign NSIS installer
```

## 🏗️ Building for Production

### Standard Build
```bash
npm run tauri:build
```
This creates:
- Executable: `Backend/target/release/search-tool.exe`
- Installer: `Backend/target/release/bundle/msi/Search Tool_*.msi`

### Signed Build (for distribution)
```bash
npm run tauri:build-signed
```
Requires environment variables:
- `SIGN_CERT` - Path to signing certificate
- `SIGN_CERT_PASS` - Certificate password

## 🔧 Configuration

### Frontend Configuration
- **Vite**: `Frontend/vite.config.js` - Build tool configuration
- **Svelte**: `Frontend/svelte.config.js` - Svelte/SvelteKit configuration
- **TypeScript**: `tsconfig.json` - TypeScript compiler settings

### Backend Configuration
- **Tauri**: `Backend/tauri.conf.json` - Application metadata and build settings
- **Rust**: `Backend/Cargo.toml` - Rust package and dependencies

## 🐛 Debugging

### Frontend Debugging
- **Browser DevTools**: Press `F12` in the application window
- **Console Logs**: View in browser console
- **Svelte DevTools**: Install browser extension for component inspection

### Backend Debugging
- **Rust Console**: Terminal output during development
- **Debug Build**: Use `npm run tauri:dev` for detailed error messages
- **Log Files**: Application logs are stored in the app directory

## 📁 Key Directories Explained

### Frontend (`Frontend/`)
Contains the Svelte/SvelteKit web application that provides the user interface. This includes all UI components, routing, state management, and frontend logic.

### Backend (`Backend/`)
Contains the Rust/Tauri application that handles system integration, file operations, and the core search functionality using ripgrep.

### Scripts (`Scripts/`)
Utility scripts for build automation, certificate management, and dependency downloading.

### Documentation (`Documentation/`)
Comprehensive project documentation including API docs, architecture diagrams, and development guides.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

[Add your license here]

## 🏷️ Version

Version 1.0.0 - Modern Architecture with Clean Separation

---

**Built with ❤️ using Tauri, Svelte, and Rust** 