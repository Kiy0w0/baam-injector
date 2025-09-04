# 🎯 Baam Injector

<div align="center">

![Baam Injector Logo](assets/icon.png)

**A modern, feature-rich Rust DLL injector with beautiful customizable GUI featuring 5 stunning themes**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows-blue.svg?style=for-the-badge&logo=windows)](https://www.microsoft.com/windows/)
[![GUI: egui](https://img.shields.io/badge/GUI-egui-green.svg?style=for-the-badge)](https://github.com/emilk/egui)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Kiy0w0/baam-injector/rust.yml?style=for-the-badge&logo=github)](https://github.com/Kiy0w0/baam-injector/actions)

[![GitHub stars](https://img.shields.io/github/stars/Kiy0w0/baam-injector?style=for-the-badge&logo=github)](https://github.com/Kiy0w0/baam-injector/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/Kiy0w0/baam-injector?style=for-the-badge&logo=github)](https://github.com/Kiy0w0/baam-injector/network)
[![GitHub issues](https://img.shields.io/github/issues/Kiy0w0/baam-injector?style=for-the-badge&logo=github)](https://github.com/Kiy0w0/baam-injector/issues)
[![GitHub release](https://img.shields.io/github/v/release/Kiy0w0/baam-injector?style=for-the-badge&logo=github)](https://github.com/Kiy0w0/baam-injector/releases)

</div>

---

> ⚠️ **Educational Use Only**
> 
> This Windows DLL injection tool is intended strictly for **educational and development purposes**, such as testing your own software. Injecting DLLs into arbitrary processes can lead to **application crashes, system instability, or detection by anti-cheat software**.
> 
> **Use responsibly and ethically.** The author is not responsible for any damage or consequences resulting from misuse.

## 📸 Screenshots

<div align="center">

<img width="801" height="599" alt="image" src="https://github.com/user-attachments/assets/06e02386-f2ab-4b52-b3b5-82ddb531f2ca" />

</div>

---

## ✨ Key Features

### 🎨 **Visual Excellence**
- **5 Beautiful Themes** with instant switching
- **Dynamic Color System** that adapts all UI elements
- **Theme Persistence** - remembers your preference
- **Smooth Animations** and responsive design
- **Modern UI** built with egui framework

### 🔍 **Smart Process Management**
- **Real-time Process Scanning** with application icons
- **Instant Search & Filter** through process list
- **Auto-refresh Option** to keep processes updated
- **Process Information Display** (PID, name, status)

### 📂 **Advanced DLL Management**
- **File Dialog Integration** for easy DLL selection
- **Persistent DLL Library** with session memory
- **DLL Validation** and compatibility checking
- **Easy Add/Remove** with visual feedback

### 🚀 **Reliable Injection System**
- **Safe DLL Injection** using CreateRemoteThread + LoadLibraryA
- **Error Handling** with detailed feedback
- **Success Notifications** via toast system
- **Process Validation** before injection

### 💾 **Session Management**
- **Configuration Persistence** - remembers DLLs and settings
- **Last Process Memory** - auto-selects previous target
- **Theme Settings** saved between sessions
- **JSON Configuration** for easy backup

### 🔔 **User Experience**
- **Toast Notifications** for all operations
- **Responsive Interface** with no freezing
- **Keyboard Navigation** support
- **Error Prevention** and validation

---


## 🚀 Getting Started

### For Users (Recommended)

1. **Download** the latest `injector.exe` from [Releases](https://github.com/Kiy0w0/baam-injector/releases)
2. **Run** the executable (no installation required)
3. **Select Theme** from the dropdown in the top bar
4. **Choose Process** from the left panel
5. **Add DLL** using the "ADD DLL" button in the right panel  
6. **Select DLL** from your library
7. **Click "INJECT DLL"** to perform injection

### For Developers (Building from Source)

**Requirements:**
- Rust Toolchain (stable)
- Git
- Windows SDK (for Windows API bindings)

**Clone & Build:**
```bash
git clone https://github.com/Kiy0w0/baam-injector.git
cd baam-injector

# Debug build
cargo run

# Release build (recommended)
cargo build --release
```

The executable will be in `target/release/injector.exe`

---

## 🛠️ Technology Stack

Built with modern Rust ecosystem:

| Component | Purpose | Version |
|-----------|---------|---------|
| **egui & eframe** | Immediate-mode GUI framework | 0.27 |
| **windows-rs** | Safe Windows API bindings | 0.60 |
| **sysinfo** | System information & process scanning | 0.33 |
| **rfd** | Native file dialogs | 0.14 |
| **serde & serde_json** | Configuration serialization | 1.0 |
| **image** | Icon processing | 0.24 |
| **anyhow** | Error handling | 1.0 |
| **webbrowser** | GitHub link integration | 0.8 |




## 📊 Performance & Safety

- **Memory Safe** - Built with Rust's ownership system
- **No Memory Leaks** - Automatic resource management
- **Fast UI** - Immediate-mode rendering at 60+ FPS
- **Small Binary** - Optimized release builds (~2MB)
- **No Dependencies** - Single executable with all assets embedded

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **🐛 Bug Reports** - Open an issue with reproduction steps
2. **💡 Feature Requests** - Suggest new themes or functionality  
3. **🔧 Pull Requests** - Submit improvements (please open an issue first)
4. **📖 Documentation** - Help improve guides and examples

### Development Setup
```bash
git clone https://github.com/Kiy0w0/baam-injector.git
cd baam-injector
cargo run --release
```

---

## 📝 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Credits & Acknowledgments

### Original Project
This project is a fork and enhancement of [**Fluffy Injector**](https://github.com/fluffysnaff/fluffy-injector) by [@fluffysnaff](https://github.com/fluffysnaff). 

Special thanks to the original author for creating the solid foundation that made this enhanced version possible. The core injection logic, process management, and initial GUI structure are based on their excellent work.

### Key Enhancements in Baam Injector
- 🎨 Complete theme system with 5 unique themes
- 🔗 **NEW:** GitHub repository integration and link
- 🎯 Enhanced UI/UX with modern design patterns
- 🔧 Improved error handling and stability
- 💾 Enhanced configuration management
- 🌟 Better visual feedback and notifications

### Dependencies
- **egui** team for the amazing immediate-mode GUI framework
- **Microsoft** for Windows API and development tools
- **Rust community** for the incredible ecosystem

---

## 🚀 Latest Updates (v0.2.0)

### 🎨 **UI/UX Improvements**
- GitHub repository link integration
- Enhanced theme system with better persistence
- Improved visual feedback and notifications
- Better configuration management

### 🔧 **Technical Enhancements**
- Simplified and more robust injection system
- Error handling improvements
- Code architecture refinements
- Better memory safety

### 🐛 **Bug Fixes**
- Fixed borrowing conflicts in process selection
- Resolved compilation warnings
- Improved stability and reliability
- Enhanced error messages

---

## 📈 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=Kiy0w0/baam-injector&type=Date)](https://star-history.com/#Kiy0w0/baam-injector&Date)

---

## 🔗 Links

- 🏠 **Homepage:** [GitHub Repository](https://github.com/Kiy0w0/baam-injector)
- 📥 **Downloads:** [Latest Releases](https://github.com/Kiy0w0/baam-injector/releases)
- 🐛 **Bug Reports:** [Issues](https://github.com/Kiy0w0/baam-injector/issues)
- 🔧 **Original Project:** [Fluffy Injector](https://github.com/fluffysnaff/fluffy-injector)

---

<div align="center">

**Made with ❤️ and 🦀 Rust**

*If you find this project useful, please consider giving it a ⭐ star!*

</div>
