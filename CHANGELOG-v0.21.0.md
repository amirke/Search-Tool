# SearchTool v0.21.0 - Release Changelog

## 🎉 **What's New in This Release**

### ✅ **UI/UX Improvements**
- **Expand/Collapse Button**: Replaced checkbox with a toggle button
  - Shows "Expanded" when results are expanded
  - Shows "Collapsed" when results are collapsed
  - Better visual feedback and user experience

### ✅ **Default Behavior Changes**
- **Results Expanded by Default**: Search results now show expanded by default
- **Persistent State**: Expand/collapse state is maintained across searches
- **Better Initial Experience**: Users see all results immediately

### ✅ **Technical Improvements**
- **Tauri v2 Compatibility**: Updated imports for Tauri v2 API
  - Fixed `@tauri-apps/api/tauri` → `@tauri-apps/api/core`
  - Resolved import errors and compatibility issues
- **Build Process**: Updated build pipeline with latest changes
- **Hot Reloading**: Improved development experience

### ✅ **Distribution Improvements**
- **Portable Package**: No-install ZIP distribution
- **SmartScreen Bypass**: Avoids Windows Defender warnings
- **Automated Setup**: `First-Run.bat` for easy WebView2 installation
- **Clear Documentation**: Updated README with latest features

### 🐛 **Bug Fixes**
- Fixed expand/collapse state not being respected on new searches
- Resolved Tauri API import compatibility issues
- Fixed development server port conflicts

### 🔧 **Developer Experience**
- Cleaned up unused imports and warnings
- Better error handling and logging
- Improved build process reliability

## 📦 **Distribution Files**

### **Main Installer** (Traditional)
- `Search Tool_0.21.0_x64-setup.exe` (3.8MB)
- NSIS installer with code signing
- May trigger SmartScreen warnings

### **Portable Version** (Recommended)
- `SearchTool-v0.21.0-Portable-Updated.zip` (7.6MB)
- No installation required
- Bypasses SmartScreen warnings
- Includes automated setup script

## 🚀 **How to Use**

### **Portable Version (Recommended)**
1. Download `SearchTool-v0.21.0-Portable-Updated.zip`
2. Extract to any folder
3. Run `First-Run.bat` OR `Search Tool.exe`
4. Enjoy!

### **Traditional Installer**
1. Download `Search Tool_0.21.0_x64-setup.exe`
2. Run installer (may show SmartScreen warning)
3. Click "More info" → "Run anyway" if prompted
4. Follow installation wizard

## 🎯 **Key Features**

- ⚡ **Fast text search** using ripgrep engine
- 🔍 **Regex support** for advanced patterns
- 📁 **Directory filtering** for targeted searches
- 🎨 **Syntax highlighting** in results
- 🖥️ **Native desktop app** with WebView2
- 📱 **Responsive UI** that works on any screen size
- 🔄 **Real-time results** with expand/collapse controls

## 📋 **System Requirements**

- **OS**: Windows 10/11 (x64)
- **Runtime**: Microsoft Edge WebView2 (auto-installed)
- **Storage**: ~20MB free space
- **Memory**: 50MB RAM (typical usage)

## 🏆 **Why This Release**

This release focuses on **user experience improvements** and **distribution reliability**:

1. **Better UX**: Toggle button is more intuitive than checkbox
2. **Faster Workflow**: Results expanded by default saves clicks
3. **Easier Distribution**: Portable version bypasses security warnings
4. **Technical Stability**: Updated to latest Tauri v2 APIs

---

**Enjoy the improved SearchTool experience! 🎉** 