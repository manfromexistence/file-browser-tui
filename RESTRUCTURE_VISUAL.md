# Visual Restructure Guide

## BEFORE (Current Structure)

```
project-root/
├── yazi-fm/
│   └── src/
│       ├── tui/              ← Your custom TUI
│       ├── app/              ← File browser stuff
│       ├── cmp/              ← File browser stuff
│       ├── confirm/          ← File browser stuff
│       ├── help/             ← File browser stuff
│       ├── input/            ← File browser stuff
│       ├── mgr/              ← File browser stuff
│       ├── notify/           ← File browser stuff
│       ├── pick/             ← File browser stuff
│       ├── spot/             ← File browser stuff
│       ├── tasks/            ← File browser stuff
│       ├── which/            ← File browser stuff
│       ├── main.rs
│       ├── dispatcher.rs
│       ├── executor.rs
│       ├── router.rs
│       └── ...
│
├── yazi-actor/               ← Separate crate
├── yazi-adapter/             ← Separate crate
├── yazi-boot/                ← Separate crate
├── yazi-config/              ← Separate crate
├── yazi-core/                ← Separate crate
├── yazi-dds/                 ← Separate crate
├── ... (20+ yazi-* crates)
└── Cargo.toml
```

**Problems:**
- ❌ TUI code mixed with file browser code
- ❌ "yazi-" prefix everywhere (not your brand)
- ❌ Hard to find your custom code
- ❌ Confusing structure

---

## AFTER (New Structure)

```
project-root/
├── src/
│   ├── menu/                 ✨ YOUR TUI MENU (flat in src/)
│   │   ├── mod.rs
│   │   ├── menu_data.rs
│   │   ├── menu_render.rs
│   │   ├── menu_navigation.rs
│   │   ├── keyboard_mappings.rs
│   │   └── submenus/
│   │
│   ├── theme.rs              ✨ YOUR TUI (flat in src/)
│   ├── render.rs             ✨ YOUR TUI
│   ├── state.rs              ✨ YOUR TUI
│   ├── animations.rs         ✨ YOUR TUI
│   ├── exit_animation.rs     ✨ YOUR TUI
│   ├── input.rs              ✨ YOUR TUI
│   │
│   ├── file_browser/         📁 ALL FILE BROWSER CODE (nested)
│   │   ├── app/              (from yazi-fm/src/app)
│   │   ├── cmp/              (from yazi-fm/src/cmp)
│   │   ├── confirm/          (from yazi-fm/src/confirm)
│   │   ├── help/             (from yazi-fm/src/help)
│   │   ├── input/            (from yazi-fm/src/input)
│   │   ├── mgr/              (from yazi-fm/src/mgr)
│   │   ├── notify/           (from yazi-fm/src/notify)
│   │   ├── pick/             (from yazi-fm/src/pick)
│   │   ├── spot/             (from yazi-fm/src/spot)
│   │   ├── tasks/            (from yazi-fm/src/tasks)
│   │   ├── which/            (from yazi-fm/src/which)
│   │   ├── executor.rs
│   │   ├── router.rs
│   │   │
│   │   ├── actor/            📦 (was yazi-actor)
│   │   ├── adapter/          � (was yazi-adapter)
│   │   ├── boot/             📦 (was yazi-boot)
│   │   ├── config/           📦 (was yazi-config)
│   │   ├── core/             📦 (was yazi-core)
│   │   ├── dds/              📦 (was yazi-dds)
│   │   ├── fs/               📦 (was yazi-fs)
│   │   ├── macro/            📦 (was yazi-macro)
│   │   ├── parser/           📦 (was yazi-parser)
│   │   ├── plugin/           📦 (was yazi-plugin)
│   │   ├── proxy/            📦 (was yazi-proxy)
│   │   ├── scheduler/        📦 (was yazi-scheduler)
│   │   ├── shared/           📦 (was yazi-shared)
│   │   ├── term/             📦 (was yazi-term)
│   │   ├── vfs/              📦 (was yazi-vfs)
│   │   ├── watcher/          📦 (was yazi-watcher)
│   │   ├── widgets/          📦 (was yazi-widgets)
│   │   └── mod.rs
│   │
│   ├── lib.rs                🎯 Main library
│   ├── main.rs               🚀 Entry point
│   ├── dispatcher.rs
│   ├── root.rs
│   ├── chat.rs
│   └── ...
│
└── Cargo.toml                📝 Clean workspace config
```

**Benefits:**
- ✅ Flat structure: TUI code directly in `src/` (no extra nesting)
- ✅ Everything file browser related in one place: `src/file_browser/`
- ✅ No "yazi" branding pollution
- ✅ Easy to find your custom code (src/menu/, src/theme.rs, etc.)
- ✅ File browser completely contained in one folder
- ✅ Supporting crates nested inside `src/file_browser/`
- ✅ Clean, professional structure
- ✅ Single workspace, everything in `src/`

---

## Import Changes

### Before:
```rust
use yazi_config::Config;
use yazi_core::Core;
use yazi_shared::Data;
use crate::app::App;
use crate::tui::Renderer;
```

### After:
```rust
use fb_config::Config;
use fb_core::Core;
use fb_shared::Data;
use crate::file_browser::app::App;
use crate::menu::Menu;        // Flat in src/
use crate::theme::ChatTheme;  // Flat in src/
```

---

## Cargo.toml Changes

### Before:
```toml
[dependencies]
yazi-actor = { path = "yazi-actor" }
yazi-config = { path = "yazi-config" }
yazi-core = { path = "yazi-core" }
```

### After:
```toml
[dependencies]
fb-actor = { path = "src/file_browser/actor" }
fb-config = { path = "src/file_browser/config" }
fb-core = { path = "src/file_browser/core" }
```

---

## Module Structure

### Main Library (src/lib.rs):
```rust
// TUI modules (flat in src/)
pub mod menu;
pub mod theme;
pub mod render;
pub mod state;
pub mod input;
pub mod animations;

// File browser module (nested)
pub mod file_browser;

// Re-exports
pub use theme::{ChatTheme, ThemeVariant};
pub use state::ChatState;
pub use render::Renderer;
pub use menu::Menu;
pub use file_browser::{Executor, Router};
```

### File Browser Module (src/file_browser/mod.rs):
```rust
// File browser UI components
pub mod app;
pub mod cmp;
pub mod confirm;
pub mod help;
pub mod input;
pub mod mgr;
pub mod notify;
pub mod pick;
pub mod spot;
pub mod tasks;
pub mod which;
pub mod executor;
pub mod router;

// Supporting crates (nested)
pub mod actor;
pub mod adapter;
pub mod config;
pub mod core;
// ... all other crates

pub use executor::Executor;
pub use router::Router;
```

---

## How to Use the Script

1. **Backup your work:**
   ```bash
   git add -A
   git commit -m "Backup before restructure"
   ```

2. **Make script executable:**
   ```bash
   chmod +x restructure.sh
   ```

3. **Run the script:**
   ```bash
   ./restructure.sh
   ```

4. **Check the results:**
   ```bash
   cargo check
   ```

5. **Fix any remaining issues:**
   - Update imports that the script missed
   - Fix module paths
   - Test compilation

6. **Clean up old folders (when satisfied):**
   ```bash
   rm -rf yazi-*/
   ```

---

## What the Script Does

1. ✅ Creates `src/tui/` and `src/file_browser/` directories
2. ✅ Moves your TUI code to `src/tui/`
3. ✅ Moves file browser code to `src/file_browser/`
4. ✅ Moves all yazi-* crates to `file_browser/` (without yazi- prefix)
5. ✅ Updates all Cargo.toml files (yazi-* → fb-*)
6. ✅ Updates all imports in Rust files
7. ✅ Creates module files (mod.rs, lib.rs)
8. ✅ Creates new workspace Cargo.toml

---

## Expected Result

After running the script and fixing any issues:

```bash
$ tree -L 2 -d
.
├── src
│   ├── menu             ← Your TUI menu (flat)
│   ├── animations       ← Your TUI animations (flat)
│   └── file_browser     ← ALL file browser code + crates
│       ├── app
│       ├── actor
│       ├── config
│       ├── core
│       └── ... (all crates nested here)
└── target

$ cargo check
   Compiling fb-shared v26.2.2 (src/file_browser/shared)
   Compiling fb-config v26.2.2 (src/file_browser/config)
   Compiling fb-core v26.2.2 (src/file_browser/core)
   ...
   Compiling dx-tui v26.2.2
    Finished dev [unoptimized + debuginfo] target(s) in 45.2s
```

---

## Troubleshooting

### Issue: "cannot find crate `yazi_*`"
**Fix:** Search and replace remaining `yazi_` references:
```bash
find src/ -name "*.rs" -exec sed -i 's/yazi_/fb_/g' {} +
```

### Issue: "unresolved import `crate::app`"
**Fix:** Update to `crate::file_browser::app`

### Issue: Module not found
**Fix:** Check that mod.rs files exist and export the modules

---

## Next Steps After Restructure

1. Update README.md with new structure
2. Update documentation
3. Rename binary from "yazi" to "dx" or "codex-tui"
4. Update config paths (~/.yazi → ~/.dx)
5. Test all functionality
6. Celebrate! 🎉
