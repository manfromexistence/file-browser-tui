# Light/Dark Mode Toggle Guide

## Overview
The TUI now supports toggling between light and dark modes for all 36 themes. Each theme has both light and dark variants extracted from `theme.json`.

## How to Use

### 1. Open Theme Menu
- Press `0` to open the Command Palette
- Select "1. Theme" to enter the theme submenu

### 2. Toggle Light/Dark Mode
- Press `T` (or `t`) to toggle between Light and Dark mode
- The current mode is displayed in the menu title: "Theme - Dark Mode" or "Theme - Light Mode"
- The toggle works instantly with live preview

### 3. Browse Themes
- Use arrow keys (`↑`/`↓`) or `j`/`k` to navigate themes
- Hover with mouse to preview themes
- Theme changes apply instantly as you navigate
- Both light and dark variants are available for each theme

### 4. Confirm Selection
- Press `Enter` or click to confirm and close the menu
- The selected theme and mode persist

## Keybindings in Theme Menu

| Key | Action |
|-----|--------|
| `↑` / `k` | Previous theme |
| `↓` / `j` | Next theme |
| `PageUp` | Jump up 10 themes |
| `PageDown` | Jump down 10 themes |
| `Home` / `g` | Jump to first theme |
| `End` / `G` | Jump to last theme |
| `T` / `t` | Toggle Light/Dark mode |
| `Enter` | Confirm and close |
| `Esc` / `Backspace` | Go back |

## Available Themes (36 total)
All themes support both light and dark modes:

1. Modern Minimal
2. T3 Chat
3. Twitter
4. Mocha Mousse
5. Bubblegum
6. Doom 64
7. Catppuccin
8. Graphite
9. Perpetuity
10. Kodama Grove
11. Cosmic Night
12. Tangerine
13. Quantum Rose
14. Nature
15. Bold Tech
16. Elegant Luxury
17. Amber Minimal
18. Supabase
19. Neo Brutalism
20. Solar Dusk
21. Claymorphism
22. Cyberpunk
23. Pastel Dreams
24. Clean Slate
25. Caffeine
26. Ocean Breeze
27. Retro Arcade
28. Midnight Bloom
29. Candyland
30. Northern Lights
31. Vintage Paper
32. Sunset Horizon
33. Starry Night
34. Claude
35. Vercel
36. Mono

## Technical Details

### State Management
- `theme_mode`: Tracks current mode (Dark/Light)
- `current_theme_name`: Tracks current theme name
- Both are updated when toggling or selecting themes

### Methods
- `toggle_theme_mode()`: Switches between light and dark mode
- `apply_theme(name, mode)`: Applies a specific theme with mode
- Theme changes update both the main UI and menu instantly

### Color Extraction
- Colors are extracted from `theme.json` using `scripts/extract-tui-themes.js`
- OKLCH colors are converted to RGB for terminal compatibility
- Both light and dark variants are stored in `tui-themes.json`

## Example Workflow

1. Press `0` → Opens Command Palette
2. Select "1. Theme" → Shows 36 themes in Dark mode
3. Press `T` → Switches to Light mode, all themes now show light variants
4. Navigate with `↓` → Preview different light themes instantly
5. Press `T` again → Back to Dark mode
6. Press `Enter` → Confirm selection and close menu

The theme system provides a seamless experience with instant visual feedback!
