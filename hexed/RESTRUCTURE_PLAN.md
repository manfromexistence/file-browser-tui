# Project Restructuring Plan
**Date:** March 24, 2026  
**Goal:** Clean separation of TUI code and file browser code

## Target Structure

```
project-root/
├── src/
│   ├── tui/              ← From yazi-fm/src/tui/ (your custom TUI)
│   │   ├── menu/         ← Your menu system
│   │   ├── theme.rs
│   │   ├── render.rs
│   │   ├── state.rs
│   │   └── ...
│   │
│   ├── file_browser/     ← All Yazi file browser code
│   │   ├── app/          ← From yazi-fm/src/app/
│   │   ├── cmp/          ← From yazi-fm/src/cmp/
│   │   ├── confirm/      ← From yazi-fm/src/confirm/
│   │   ├── help/         ← From yazi-fm/src/help/
│   │   ├── input/        ← From yazi-fm/src/input/
│   │   ├── mgr/          ← From yazi-fm/src/mgr/
│   │   ├── notify/       ← From yazi-fm/src/notify/
│   │   ├── pick/         ← From yazi-fm/src/pick/
│   │   ├── spot/         ← From yazi-fm/src/spot/
│   │   ├── tasks/        ← From yazi-fm/src/tasks/
│   │   ├── which/        ← From yazi-fm/src/which/
│   │   ├── executor.rs   ← From yazi-fm/src/executor.rs
│   │   ├── router.rs     ← From yazi-fm/src/router.rs
│   │   └── mod.rs        ← New module file
│   │
│   ├── main.rs           ← From yazi-fm/src/main.rs
│   ├── dispatcher.rs     ← From yazi-fm/src/dispatcher.rs
│   ├── root.rs           ← From yazi-fm/src/root.rs
│   ├── panic.rs          ← From yazi-fm/src/panic.rs
│   ├── signals.rs        ← From yazi-fm/src/signals.rs
│   ├── logs.rs           ← From yazi-fm/src/logs.rs
│   ├── chat.rs           ← From yazi-fm/src/chat.rs
│   ├── chat_input.rs     ← From yazi-fm/src/chat_input.rs
│   ├── chat_components.rs← From yazi-fm/src/chat_components.rs
│   └── llm.rs            ← From yazi-fm/src/llm.rs
│
├── file_browser/         ← All yazi-* crates (renamed, no yazi prefix)
│   ├── actor/            ← From yazi-actor/
│   ├── adapter/          ← From yazi-adapter/
│   ├── binding/          ← From yazi-binding/
│   ├── boot/             ← From yazi-boot/
│   ├── build/            ← From yazi-build/
│   ├── cli/              ← From yazi-cli/
│   ├── codegen/          ← From yazi-codegen/
│   ├── config/           ← From yazi-config/
│   ├── core/             ← From yazi-core/
│   ├── dds/              ← From yazi-dds/
│   ├── emulator/         ← From yazi-emulator/
│   ├── ffi/              ← From yazi-ffi/
│   ├── fs/               ← From yazi-fs/
│   ├── macro/            ← From yazi-macro/
│   ├── packing/          ← From yazi-packing/
│   ├── parser/           ← From yazi-parser/
│   ├── plugin/           ← From yazi-plugin/
│   ├── proxy/            ← From yazi-proxy/
│   ├── scheduler/        ← From yazi-scheduler/
│   ├── sftp/             ← From yazi-sftp/
│   ├── shared/           ← From yazi-shared/
│   ├── shim/             ← From yazi-shim/
│   ├── term/             ← From yazi-term/
│   ├── tty/              ← From yazi-tty/
│   ├── vfs/              ← From yazi-vfs/
│   ├── watcher/          ← From yazi-watcher/
│   └── widgets/          ← From yazi-widgets/
│
├── Cargo.toml            ← Updated workspace config
└── ...
```

## Key Changes

1. **Root `src/` folder contains:**
   - `tui/` - Your custom TUI code (menu system, themes, etc.)
   - `file_browser/` - Yazi file browser integration code
   - Top-level files (main.rs, dispatcher.rs, etc.)

2. **`file_browser/` crate folder contains:**
   - All yazi-* crates renamed without the "yazi-" prefix
   - Each becomes a sub-crate in the workspace

3. **Clean separation:**
   - TUI code is isolated and easy to maintain
   - File browser code is contained in its own namespace
   - No "yazi" naming pollution in your codebase

## Migration Steps

### Step 1: Create new structure
### Step 2: Move yazi-fm/src/tui/ to src/tui/
### Step 3: Move yazi-fm/src/* (except tui) to src/file_browser/
### Step 4: Move all yazi-* crates to file_browser/*
### Step 5: Update all imports and module paths
### Step 6: Update Cargo.toml workspace configuration
### Step 7: Test compilation

