#!/usr/bin/env node

/**
 * Add DX theme (the current default green theme) to the top of theme.json
 */

const fs = require('fs');
const path = require('path');

// DX theme definition based on the hardcoded dark_fallback and light_fallback
const dxTheme = {
  "name": "dx",
  "type": "registry:style",
  "title": "DX",
  "description": "The default DX theme with green accents.",
  "css": {
    "@layer base": {
      "body": {
        "letter-spacing": "var(--tracking-normal)"
      }
    }
  },
  "cssVars": {
    "theme": {
      "font-sans": "Inter, sans-serif",
      "font-mono": "JetBrains Mono, monospace",
      "font-serif": "Source Serif 4, serif",
      "radius": "0.5rem",
      "tracking-tighter": "calc(var(--tracking-normal) - 0.05em)",
      "tracking-tight": "calc(var(--tracking-normal) - 0.025em)",
      "tracking-wide": "calc(var(--tracking-normal) + 0.025em)",
      "tracking-wider": "calc(var(--tracking-normal) + 0.05em)",
      "tracking-widest": "calc(var(--tracking-normal) + 0.1em)"
    },
    "light": {
      "background": "oklch(0.99 0 0)",      // rgb(252, 252, 252)
      "foreground": "oklch(0 0 0)",         // rgb(0, 0, 0)
      "card": "oklch(1 0 0)",               // rgb(255, 255, 255)
      "card-foreground": "oklch(0 0 0)",    // rgb(0, 0, 0)
      "popover": "oklch(0.99 0 0)",         // rgb(252, 252, 252)
      "popover-foreground": "oklch(0 0 0)", // rgb(0, 0, 0)
      "primary": "oklch(0.55 0.20 145)",    // rgb(0, 160, 60) - green
      "primary-foreground": "oklch(1 0 0)", // rgb(255, 255, 255)
      "secondary": "oklch(0.92 0 0)",       // rgb(235, 235, 235)
      "secondary-foreground": "oklch(0 0 0)", // rgb(0, 0, 0)
      "muted": "oklch(0.96 0 0)",           // rgb(245, 245, 245)
      "muted-foreground": "oklch(0.32 0 0)", // rgb(82, 82, 82)
      "accent": "oklch(0.55 0.20 145)",     // rgb(0, 160, 60) - green
      "accent-foreground": "oklch(1 0 0)",  // rgb(255, 255, 255)
      "destructive": "oklch(0.60 0.22 25)", // rgb(229, 75, 79)
      "destructive-foreground": "oklch(1 0 0)", // rgb(255, 255, 255)
      "border": "oklch(0.89 0 0)",          // rgb(228, 228, 228)
      "input": "oklch(0.92 0 0)",           // rgb(235, 235, 235)
      "ring": "oklch(0.55 0.20 145)",       // rgb(0, 160, 60) - green
      "chart-1": "oklch(0.55 0.20 145)",
      "chart-2": "oklch(0.50 0.18 200)",
      "chart-3": "oklch(0.45 0.16 250)",
      "chart-4": "oklch(0.40 0.14 300)",
      "chart-5": "oklch(0.35 0.12 350)",
      "radius": "0.5rem"
    },
    "dark": {
      "background": "oklch(0 0 0)",         // rgb(0, 0, 0)
      "foreground": "oklch(1 0 0)",         // rgb(255, 255, 255)
      "card": "oklch(0.04 0 0)",            // rgb(9, 9, 9)
      "card-foreground": "oklch(1 0 0)",    // rgb(255, 255, 255)
      "popover": "oklch(0.07 0 0)",         // rgb(18, 18, 18)
      "popover-foreground": "oklch(1 0 0)", // rgb(255, 255, 255)
      "primary": "oklch(0.65 0.22 145)",    // rgb(0, 201, 80) - green
      "primary-foreground": "oklch(1 0 0)", // rgb(255, 255, 255)
      "secondary": "oklch(0.13 0 0)",       // rgb(34, 34, 34)
      "secondary-foreground": "oklch(1 0 0)", // rgb(255, 255, 255)
      "muted": "oklch(0.11 0 0)",           // rgb(29, 29, 29)
      "muted-foreground": "oklch(0.64 0 0)", // rgb(164, 164, 164)
      "accent": "oklch(0.65 0.22 145)",     // rgb(0, 201, 80) - green
      "accent-foreground": "oklch(1 0 0)",  // rgb(255, 255, 255)
      "destructive": "oklch(0.68 0.20 25)", // rgb(255, 91, 91)
      "destructive-foreground": "oklch(0 0 0)", // rgb(0, 0, 0)
      "border": "oklch(0.14 0 0)",          // rgb(36, 36, 36)
      "input": "oklch(0.20 0 0)",           // rgb(51, 51, 51)
      "ring": "oklch(0.64 0 0)",            // rgb(164, 164, 164)
      "chart-1": "oklch(0.65 0.22 145)",
      "chart-2": "oklch(0.60 0.20 200)",
      "chart-3": "oklch(0.55 0.18 250)",
      "chart-4": "oklch(0.50 0.16 300)",
      "chart-5": "oklch(0.45 0.14 350)",
      "radius": "0.5rem"
    }
  }
};

function main() {
  const themeJsonPath = path.join(__dirname, '..', 'theme.json');
  
  console.log('Reading theme.json...');
  const themeData = JSON.parse(fs.readFileSync(themeJsonPath, 'utf8'));
  
  console.log(`Found ${themeData.items.length} themes`);
  
  // Check if DX theme already exists
  const dxIndex = themeData.items.findIndex(t => t.name === 'dx');
  
  if (dxIndex !== -1) {
    console.log('DX theme already exists, removing old version...');
    themeData.items.splice(dxIndex, 1);
  }
  
  // Insert DX theme at the beginning
  themeData.items.unshift(dxTheme);
  
  console.log('Added DX theme at the top');
  
  // Write back to theme.json
  fs.writeFileSync(themeJsonPath, JSON.stringify(themeData, null, 2), 'utf8');
  
  console.log('✓ Done! Updated theme.json');
  console.log(`  DX theme is now at position 1`);
  console.log(`  Total themes: ${themeData.items.length}`);
}

main();
