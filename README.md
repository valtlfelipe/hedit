# Hedit

A sleek and intuitive hosts file editor built with Tauri, Vue.js, and Rust. Simplify managing your `/etc/hosts` file with a modern, cross-platform application.

![](https://github.com/user-attachments/assets/fa0e871a-edbf-45b8-925c-8e55dbf6c08e)

---

## 🚧 Work in Progress 🚧

**Please Note:** This project is currently under active development. Features may be incomplete, and bugs are expected. Your feedback and contributions are highly welcome!

---

## 🍎💻 Cross-Platform Support

This application is available and tested on macOS and Linux. Windows support is planned for future releases.

---

## ✨ Features

*   **Intuitive UI:** Clean and user-friendly interface for easy hosts file management with Light & Dark mode.
*   **Fast & Secure:** Built with Rust (Tauri) for performance and system-level access.
*   **Multiple Files:** Manage multiple files and activate the one you need.

---

## ⬇️ Installation & Usage (macOS & Linux)

Since this application is not yet signed, you'll need to follow these steps to open it:

### macOS:
1.  **Download the latest `.dmg` or `.app` file** from the [Releases page](https://github.com/valtlfelipe/hedit/releases) (link will be updated once releases are available).
2.  **Drag the application** to your Applications folder.
3.  **Right-click** on the `Hedit.app` icon in your Applications folder.
4.  Select **"Open"** from the context menu.
5.  If a warning dialog appears stating "Hedit.app cannot be opened because it is from an unidentified developer," click **"Open"** again.
6.  If the app still doesn't open, go to **System Settings** (or System Preferences) > **Privacy & Security**.
7.  Scroll down to the "Security" section. You should see a message like "Hedit.app was blocked from opening because it is not from an identified developer."
8.  Click the **"Open Anyway"** button next to this message.
9.  You will be prompted to confirm. Click **"Open"**.

The application should now launch successfully.

### Linux:
1. **Download the latest `.deb` or `.AppImage` file** from the [Releases page](https://github.com/valtlfelipe/hedit/releases).
2. **For .deb files:**
   - Double-click the file to open it with your package manager, or
   - Install via terminal: `sudo dpkg -i hedit_x.x.x_amd64.deb`
3. **For .AppImage files:**
   - Make the file executable: `chmod +x hedit_x.x.x.AppImage`
   - Run the application: `./hedit_x.x.x.AppImage`
   
**Note:** On some Linux distributions, you may need to install additional dependencies:
- For Debian/Ubuntu-based systems: `sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0`