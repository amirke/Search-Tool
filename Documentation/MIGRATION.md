# Migration Guide: Restructured Search Tool Project

## Overview

This document explains the migration from the old flat project structure to the new organized Frontend/Backend structure.

## 🔄 What Changed

### Old Structure → New Structure

```
OLD:                          NEW:
Search-Tool/                  SearchTool/
├── src/                 →    ├── Frontend/src/
├── src-tauri/          →    ├── Backend/
├── static/             →    ├── Frontend/static/
├── *.ps1               →    ├── Scripts/
├── *.md                →    ├── Documentation/
├── icons/              →    ├── icons/ (shared)
├── package.json        →    ├── package.json (updated)
└── config files        →    └── respective directories
```

### Key Changes

1. **Frontend Separation**: All frontend code moved to `Frontend/` directory
   - `src/` → `Frontend/src/`
   - `static/` → `Frontend/static/`
   - Frontend configs remain in `Frontend/`

2. **Backend Separation**: All backend code moved to `Backend/` directory
   - `src-tauri/` → `Backend/`
   - All Rust code and Tauri configs in `Backend/`

3. **Scripts Organization**: All PowerShell scripts moved to `Scripts/`
   - `*.ps1` files → `Scripts/`
   - Build and automation scripts centralized

4. **Documentation Organization**: All documentation moved to `Documentation/`
   - `*.md` files → `Documentation/`
   - Comprehensive docs in one place

## 📝 Configuration Updates

### Package.json Scripts Updated

**Before:**
```json
{
  "scripts": {
    "dev": "vite dev",
    "tauri": "tauri",
    "start": "powershell -File src-tauri/build.ps1"
  }
}
```

**After:**
```json
{
  "scripts": {
    "dev": "cd Frontend && vite dev",
    "tauri": "cd Backend && tauri",
    "tauri:dev": "cd Backend && tauri dev",
    "start": "powershell -File Backend/build.ps1"
  }
}
```

### Tauri Configuration Updated

**Before (src-tauri/tauri.conf.json):**
```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "distDir": "../build"
  }
}
```

**After (Backend/tauri.conf.json):**
```json
{
  "build": {
    "beforeDevCommand": "cd ../Frontend && npm run dev",
    "beforeBuildCommand": "cd ../Frontend && npm run build",
    "distDir": "../Frontend/build"
  }
}
```

## 🛠️ Development Workflow Changes

### Before (Old Structure)
```bash
# Development
npm run tauri dev

# Building
npm run tauri build

# Scripts
powershell -File create-cert.ps1
```

### After (New Structure)
```bash
# Setup (first time)
powershell -ExecutionPolicy Bypass -File setup.ps1

# Development
npm run tauri:dev

# Building
npm run tauri:build

# Scripts
powershell -File Scripts/create-cert.ps1
```

## 🎯 Benefits of New Structure

### ✅ Improved Organization
- **Clear separation** between frontend and backend code
- **Logical grouping** of related files
- **Easier navigation** for developers

### ✅ Better Maintainability
- **Isolated dependencies** for frontend and backend
- **Cleaner build processes**
- **Simplified debugging**

### ✅ Enhanced Scalability
- **Modular architecture** allows for easier expansion
- **Independent development** of frontend and backend
- **Better team collaboration**

### ✅ Professional Structure
- **Industry-standard organization**
- **Improved documentation**
- **Better onboarding for new developers**

## 🔧 Path Reference Updates

If you have any custom scripts or configurations, update these paths:

| Old Path | New Path |
|----------|----------|
| `src/` | `Frontend/src/` |
| `src-tauri/` | `Backend/` |
| `static/` | `Frontend/static/` |
| `*.ps1` | `Scripts/*.ps1` |
| `*.md` | `Documentation/*.md` |

## 🚀 Getting Started with New Structure

1. **Clone the new repository:**
   ```bash
   git clone <new-repo-url>
   cd SearchTool
   ```

2. **Run the setup script:**
   ```bash
   powershell -ExecutionPolicy Bypass -File setup.ps1
   ```

3. **Start development:**
   ```bash
   npm run tauri:dev
   ```

## 🐛 Common Issues and Solutions

### Issue: "Cannot find module" errors
**Solution:** Make sure you're running commands from the project root and that all dependencies are installed via `npm install`.

### Issue: Build fails with path errors
**Solution:** Check that the `tauri.conf.json` has the correct `distDir` path (`../Frontend/build`).

### Issue: Scripts not found
**Solution:** Update any custom scripts to use the new `Scripts/` directory path.

## 📞 Support

If you encounter any issues during migration:

1. Check this migration guide first
2. Refer to the main `README.md`
3. Check the `Documentation/` folder for detailed guides
4. Create an issue in the repository

## 🎉 Welcome to the New Structure!

The new organization makes the project more professional, maintainable, and easier to work with. Happy coding! 