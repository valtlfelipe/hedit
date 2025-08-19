# Hedit - Project Context for Qwen Code

This document provides essential context for Qwen Code to understand and assist with the Hedit project.

## Project Overview

**Hedit** is a cross-platform desktop application for managing the system's `/etc/hosts` file. It provides a user-friendly interface built with modern web technologies (Vue.js) and a secure, performant backend using Rust (Tauri framework). The main goal is to simplify creating, editing, and activating different versions of the hosts file.

### Key Technologies

- **Frontend:** Vue 3 (Composition API), TypeScript, Tailwind CSS, Vite.
- **Backend:** Rust, Tauri.
- **Build System:** Bun (for frontend scripts), Cargo (for Rust), Tauri CLI.
- **Packaging:** Tauri (creates native macOS `.app`, `.dmg`; Windows and Linux planned).

### Core Architecture

- **Tauri App:** The main executable is a Tauri application. It embeds a web view to render the Vue.js frontend.
- **Frontend (UI):** The `src/` directory contains the Vue.js application. It manages the UI, user interactions, and state (via Pinia/Vue stores).
- **Backend (Rust):** The `src-tauri/` directory contains the Rust code. This handles system-level operations like reading/writing files (`/etc/hosts`, app data), creating native menus, managing app lifecycle, and exposing safe APIs to the frontend via Tauri commands and events.
- **Data Storage:** User-created hosts files are stored in the app's data directory (`AppData`), managed by Tauri's filesystem plugin (`@tauri-apps/plugin-fs`). Metadata about these files (names, IDs, active status) is stored using the Tauri Store plugin (`@tauri-apps/plugin-store`).
- **File Activation:** When a user "activates" a file, its content is written to the system's `/etc/hosts` file (requires appropriate permissions, handled by Tauri/Rust).

### Current Status & Platform Support

- **Work in Progress:** Features are incomplete, and bugs are expected.
- **macOS Focus:** Currently developed and tested primarily for macOS, utilizing macOS-specific APIs (like the transparent title bar). Windows and Linux support is planned.

## Building and Running

### Prerequisites

- **Rust & Cargo:** Required for the Tauri backend. [Install Rust](https://www.rust-lang.org/).
- **Bun:** Used for frontend tooling and scripts. [Install Bun](https://bun.sh/).
- **Tauri CLI:** Usually installed via `bun` as a dev dependency (`@tauri-apps/cli`).

### Development Commands

1.  **Install Dependencies:**
    ```bash
    bun install
    ```
2.  **Run in Development Mode:**
    This starts the Vite dev server and the Tauri application, allowing hot-reloading of the frontend.
    ```bash
    bun run tauri dev
    ```
3.  **Build for Production:**
    This compiles the Vue frontend and bundles it with the Rust backend into a final executable/native app.
    ```bash
    bun run tauri build
    ```
4.  **Type Checking (Frontend):**
    ```bash
    bun run check
    ```

### File Structure (Key Directories & Files)

- `src/`: Vue.js frontend source code.
- `src-tauri/`: Rust/Tauri backend source code.
- `src-tauri/src/main.rs`: Rust entry point.
- `src-tauri/src/lib.rs`: Contains the `run()` function that initializes the Tauri app.
- `src-tauri/Cargo.toml`: Rust dependencies and configuration.
- `src-tauri/tauri.conf.json`: Tauri configuration (build settings, app metadata, bundling options).
- `src/App.vue`: Main Vue application component.
- `src/stores/`: Vue stores for managing application state (e.g., `files.ts` for hosts file management).
- `package.json`: Frontend dependencies, scripts.
- `vite.config.ts`: Vite build configuration.
- `tailwind.config.js`: Tailwind CSS configuration.

## Development Conventions

- **Language:** TypeScript for frontend, Rust for backend.
- **Styling:** Tailwind CSS.
- **Frontend Framework:** Vue 3 with Composition API (`<script setup>`).
- **State Management:** Vue's reactivity (`reactive`) and potentially Pinia (for stores like `src/stores/files.ts`).
- **UI Components:** Custom components built with Vue and Tailwind (e.g., `MacOSWindow`, `Sidebar`, `Toolbar`, `CodeEditor`).
- **File I/O:** Handled via Tauri plugins (`@tauri-apps/plugin-fs`, `@tauri-apps/plugin-store`) for security.
- **Linting/Formatting:** Likely uses Biome (configured via `biome.json` and `@biomejs/biome`).
