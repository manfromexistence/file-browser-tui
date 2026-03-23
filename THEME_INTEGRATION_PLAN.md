# Theme Integration Plan

## Completed
✅ Created `scripts/extract-tui-themes.js` - Extracts TUI-relevant colors from theme.json
✅ Generated `tui-themes.json` - Contains 36 themes with RGB color values for dark/light modes

## Next Steps

### 1. Update Theme Loading System (yazi-fm/src/tui/theme.rs)

Add functionality to:
- Load themes from `tui-themes.json` at runtime
- Parse JSON and convert RGB arrays to `Color::Rgb`
- Provide a list of available themes
- Switch between themes dynamically

```rust
// Add to theme.rs:
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize)]
struct TuiThemes {
    version: String,
    themes: Vec<ThemeDefinition>,
}

#[derive(Debug, Deserialize, Clone)]
struct ThemeDefinition {
    name: String,
    title: String,
    description: String,
    dark: Option<ThemeColors>,
    light: Option<ThemeColors>,
}

#[derive(Debug, Deserialize, Clone)]
struct ThemeColors {
    bg: [u8; 3],
    fg: [u8; 3],
    card: [u8; 3],
    // ... all other colors
}

impl ChatTheme {
    pub fn load_themes() -> Result<Vec<ThemeDefinition>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("tui-themes.json")?;
        let themes: TuiThemes = serde_json::from_str(&content)?;
        Ok(themes.themes)
    }
    
    pub fn from_definition(def: &ThemeDefinition, variant: ThemeVariant) -> Option<Self> {
        let colors = match variant {
            ThemeVariant::Dark => def.dark.as_ref()?,
            ThemeVariant::Light => def.light.as_ref()?,
        };
        
        Some(Self {
            variant,
            bg: Color::Rgb(colors.bg[0], colors.bg[1], colors.bg[2]),
            fg: Color::Rgb(colors.fg[0], colors.fg[1], colors.fg[2]),
            // ... map all colors
        })
    }
}
```

### 2. Update Theme Menu (yazi-fm/src/tui/menu.rs)

Modify the Theme submenu to:
- Load available themes from `tui-themes.json`
- Display theme names dynamically
- Handle theme selection

```rust
// In menu.rs, update Theme submenu generation:
fn load_theme_submenu() -> Vec<(String, String)> {
    let mut items = vec![("1. Dark Mode".to_string(), String::new())];
    
    if let Ok(themes) = ChatTheme::load_themes() {
        for (idx, theme) in themes.iter().enumerate() {
            items.push((
                format!("{}. {}", idx + 2, theme.title),
                String::new()
            ));
        }
    }
    
    items
}
```

### 3. Add Theme State Management

Create a global theme state that can be updated:
- Store current theme name
- Provide theme switching function
- Persist theme choice to config file

### 4. Wire Up Theme Selection

In the menu selection handler:
- Detect when a theme is selected
- Load the new theme
- Apply it to all TUI components
- Save the selection

## Files to Modify

1. `yazi-fm/src/tui/theme.rs` - Add theme loading and switching
2. `yazi-fm/src/tui/menu.rs` - Update Theme submenu to list themes dynamically
3. `yazi-fm/Cargo.toml` - Add `serde_json` dependency if not present
4. Create theme config file to persist user's theme choice

## Testing

1. Run the TUI and navigate to Theme menu
2. Verify all 36 themes are listed
3. Select different themes and verify colors change
4. Restart TUI and verify theme persists

## Usage

```bash
# Extract themes from theme.json
node scripts/extract-tui-themes.js

# This generates tui-themes.json which the TUI reads at runtime
```
