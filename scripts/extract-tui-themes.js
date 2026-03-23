#!/usr/bin/env node

/**
 * Extract TUI-compatible themes from theme.json
 * 
 * This script reads theme.json and extracts only the color values
 * that can be used in a terminal UI (TUI), converting them to RGB
 * format that Ratatui can understand.
 */

const fs = require('fs');
const path = require('path');

// Parse OKLCH color string to RGB
function oklchToRgb(oklchStr) {
  // Parse oklch(L C H) format
  const match = oklchStr.match(/oklch\(([\d.]+)\s+([\d.]+)\s+([\d.]+)\)/);
  if (!match) {
    console.warn(`Could not parse OKLCH: ${oklchStr}`);
    return [0, 0, 0];
  }

  const [, L, C, H] = match.map(Number);
  
  // Convert OKLCH to RGB (simplified conversion)
  // This is an approximation - for production use a proper color library
  const l = L * 100;
  const c = C * 100;
  const h = H;
  
  // Convert to Lab
  const a = c * Math.cos(h * Math.PI / 180);
  const b = c * Math.sin(h * Math.PI / 180);
  
  // Lab to XYZ
  const fy = (l + 16) / 116;
  const fx = a / 500 + fy;
  const fz = fy - b / 200;
  
  const xr = fx > 0.206897 ? fx ** 3 : (fx - 16/116) / 7.787;
  const yr = fy > 0.206897 ? fy ** 3 : (fy - 16/116) / 7.787;
  const zr = fz > 0.206897 ? fz ** 3 : (fz - 16/116) / 7.787;
  
  const x = xr * 95.047;
  const y = yr * 100.000;
  const z = zr * 108.883;
  
  // XYZ to RGB
  let r = x *  3.2406 + y * -1.5372 + z * -0.4986;
  let g = x * -0.9689 + y *  1.8758 + z *  0.0415;
  let bl = x *  0.0557 + y * -0.2040 + z *  1.0570;
  
  // Gamma correction
  r = r > 0.0031308 ? 1.055 * (r ** (1/2.4)) - 0.055 : 12.92 * r;
  g = g > 0.0031308 ? 1.055 * (g ** (1/2.4)) - 0.055 : 12.92 * g;
  bl = bl > 0.0031308 ? 1.055 * (bl ** (1/2.4)) - 0.055 : 12.92 * bl;
  
  // Clamp and convert to 0-255
  r = Math.max(0, Math.min(255, Math.round(r * 255)));
  g = Math.max(0, Math.min(255, Math.round(g * 255)));
  bl = Math.max(0, Math.min(255, Math.round(bl * 255)));
  
  return [r, g, bl];
}

// Extract TUI-relevant colors from a theme
function extractTuiColors(theme) {
  const darkColors = theme.cssVars.dark || {};
  const lightColors = theme.cssVars.light || {};
  
  const extractMode = (colors) => ({
    background: colors.background || 'oklch(0 0 0)',
    foreground: colors.foreground || 'oklch(1 0 0)',
    card: colors.card || colors.background || 'oklch(0 0 0)',
    card_foreground: colors['card-foreground'] || colors.foreground || 'oklch(1 0 0)',
    primary: colors.primary || 'oklch(0.5 0.2 200)',
    primary_foreground: colors['primary-foreground'] || 'oklch(1 0 0)',
    secondary: colors.secondary || 'oklch(0.3 0 0)',
    secondary_foreground: colors['secondary-foreground'] || 'oklch(1 0 0)',
    muted: colors.muted || 'oklch(0.3 0 0)',
    muted_foreground: colors['muted-foreground'] || 'oklch(0.7 0 0)',
    accent: colors.accent || colors.primary || 'oklch(0.5 0.2 200)',
    accent_foreground: colors['accent-foreground'] || 'oklch(1 0 0)',
    destructive: colors.destructive || 'oklch(0.6 0.2 25)',
    destructive_foreground: colors['destructive-foreground'] || 'oklch(1 0 0)',
    border: colors.border || 'oklch(0.3 0 0)',
    input: colors.input || colors.border || 'oklch(0.3 0 0)',
    ring: colors.ring || colors.primary || 'oklch(0.5 0.2 200)',
  });
  
  return {
    name: theme.name,
    title: theme.title,
    description: theme.description,
    dark: extractMode(darkColors),
    light: extractMode(lightColors),
  };
}

// Convert OKLCH colors to RGB in a theme
function convertThemeToRgb(theme) {
  const convertMode = (mode) => {
    const rgb = {};
    for (const [key, value] of Object.entries(mode)) {
      const [r, g, b] = oklchToRgb(value);
      rgb[key] = { r, g, b };
    }
    return rgb;
  };
  
  return {
    name: theme.name,
    title: theme.title,
    description: theme.description,
    dark: convertMode(theme.dark),
    light: convertMode(theme.light),
  };
}

// Main execution
function main() {
  const themeJsonPath = path.join(__dirname, '..', 'theme.json');
  const outputPath = path.join(__dirname, '..', 'tui-themes.json');
  
  console.log('Reading theme.json...');
  const themeData = JSON.parse(fs.readFileSync(themeJsonPath, 'utf8'));
  
  console.log(`Found ${themeData.items.length} themes`);
  
  const tuiThemes = themeData.items.map(theme => {
    console.log(`Processing: ${theme.title}`);
    const extracted = extractTuiColors(theme);
    return convertThemeToRgb(extracted);
  });
  
  const output = {
    version: '1.0.0',
    themes: tuiThemes,
  };
  
  console.log(`Writing ${tuiThemes.length} themes to tui-themes.json...`);
  fs.writeFileSync(outputPath, JSON.stringify(output, null, 2), 'utf8');
  
  console.log('✓ Done! Created tui-themes.json');
  console.log(`  Themes: ${tuiThemes.length}`);
  console.log(`  Output: ${outputPath}`);
}

main();
