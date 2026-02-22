# AGENTS.md - Developer Guide for Hedit

This file provides guidelines for AI agents working on the Hedit codebase.

## Project Overview

Hedit is a cross-platform hosts file editor built with:
- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Tauri 2 (Rust)
- **Styling**: Tailwind CSS v4
- **Linting/Formatting**: Biome
- **Package Manager**: Bun (>=1.2.0)

---

## Build, Lint, and Test Commands

### Frontend (JavaScript/TypeScript)

```bash
# Install dependencies
bun install

# Run development server
bun run dev

# Type-check only
bun run check

# Build for production
bun run build

# Preview production build
bun run preview

# Lint with Biome
bun run lint

# Auto-fix lint issues
bun run lint:fix

# Format code with Biome
bun run format

# Format and write changes
bun run format:write
```

### Backend (Rust/Tauri)

```bash
# Run Tauri in development mode
bun run tauri dev

# Build Tauri application
bun run tauri build

# Check Rust code (clippy + fmt)
cd src-tauri && cargo check

# Format Rust code
cd src-tauri && cargo fmt

# Lint Rust code
cd src-tauri && cargo clippy
```

### Running a Single Test

This project does not currently have a test suite. If tests are added:

```bash
# Frontend tests (when implemented)
bun run test

# Run single test file
bun run test -- filename.spec.ts

# Rust tests
cd src-tauri && cargo test
```

---

## Code Style Guidelines

### General Principles

- Keep files focused and modular
- Use meaningful, descriptive names
- Handle errors explicitly with user feedback
- Prefer async/await over raw promises in TypeScript

### TypeScript / Vue

**Formatting** (Biome):
- Indent: 2 spaces
- Line width: 100 characters
- Quote style: single quotes (`'`)
- Semicolons: as-needed

**TypeScript**:
- Enable strict mode (`strict: true` in tsconfig)
- Always type function parameters and return types
- Use interfaces for object shapes, enums for unions
- Prefer `const` over `let`, avoid `var`

**Imports**:
- Group imports in this order: external libs, internal modules, relative imports
- Use named imports: `import { something } from 'module'`
- No barrel exports (`index.ts`) - import directly from files

**Vue Components**:
- Use Composition API (`<script setup>`)
- Props should be typed with `defineProps<{...}>()`
- Emit types with `const emit = defineEmits<{...}>()`

**Error Handling**:
- Use try/catch with user-facing error messages via `toast.error()`
- Log errors with `console.error()` for debugging
- Check error type: `error instanceof Error ? error.message : String(error)`

### Rust

**Formatting**:
- Run `cargo fmt` before committing
- Follow standard Rust idioms

**Error Handling**:
- Use `Result<T, String>` for Tauri commands (converts to JS error)
- Propagate errors with `?` operator
- Provide meaningful error messages

**Dependencies**:
- Keep dependencies minimal
- Use `tokio` for async runtime

### Tailwind CSS v4

- Use utility classes for styling
- Avoid custom CSS when utilities suffice
- Use CSS variables for theming in `src/style.css`

---

## Project Structure

```
hedit/
├── src/                    # Vue frontend
│   ├── components/         # Vue components
│   │   └── settings/      # Settings panel components
│   ├── composables/       # Vue composables (hooks)
│   ├── stores/            # Pinia/Vue stores
│   └── utils/             # Utility functions
├── src-tauri/             # Rust backend
│   └── src/
│       ├── lib.rs         # Main app setup
│       ├── files.rs       # File operations
│       ├── hosts_parser.rs
│       └── ...
├── biome.json             # Biome config
├── package.json           # Frontend deps
└── Cargo.toml             # Rust deps
```

---

## Common Development Tasks

### Adding a New Tauri Command

1. Create function in `src-tauri/src/*.rs` with `#[command]` attribute
2. Return `Result<T, String>` for error handling
3. Register in `lib.rs` `.invoke_handler()` macro
4. Call from frontend via `@tauri-apps/api/core` `invoke()`

### Adding a New Vue Component

1. Create `src/components/ComponentName.vue`
2. Use `<script setup lang="ts">`
3. Import and use in parent component

### Adding a New Setting

1. Add to `src-tauri/src/settings_store.rs`
2. Update settings UI in `src/components/settings/`
3. Use `settings_store::get_settings_store_config_*` helpers

---

## Git Conventions

- Commit messages: use clear, imperative mood
- Pull request title: short description of change
- Branch naming: `feature/description` or `fix/description`

---

## Notes

- Requires macOS for full development (uses macOS-private-api)
- Some features require elevated privileges (writing to `/etc/hosts`)
- App uses Tauri plugins for: fs, store, http, os, autostart, opener
