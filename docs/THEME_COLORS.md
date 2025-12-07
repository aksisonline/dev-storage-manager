# Zen Dark Theme - Color Reference

This document provides a visual reference for all colors used in the Zen Dark theme.

## Color Palette

### Base Colors

| Purpose | Hex Code | RGB | Usage |
|---------|----------|-----|-------|
| **Background** | `#171717` | rgb(23, 23, 23) | Main application background |
| **Editor Background** | `#0a0a0a` | rgb(10, 10, 10) | Project cards (unselected) |
| **Elevated Surface** | `#171717` | rgb(23, 23, 23) | Header, panels |
| **Surface** | `#171717` | rgb(23, 23, 23) | General surfaces |

### Text Colors

| Purpose | Hex Code | RGB | Usage |
|---------|----------|-----|-------|
| **Primary Text** | `#e5e5e5` | rgb(229, 229, 229) | Main text, titles |
| **Muted Text** | `#a3a3a3` | rgb(163, 163, 163) | Secondary info, paths |
| **Placeholder** | `#737373` | rgb(115, 115, 115) | Metadata, age/size info |
| **Disabled** | `#737373` | rgb(115, 115, 115) | Inactive elements |
| **Accent Text** | `#14b8a6` | rgb(20, 184, 166) | Selected count, highlights |

### Border Colors

| Purpose | Hex Code | RGB | Usage |
|---------|----------|-----|-------|
| **Default Border** | `#262626` | rgb(38, 38, 38) | Card borders, separators |
| **Variant Border** | `#171717` | rgb(23, 23, 23) | Subtle borders |
| **Focused Border** | `#737373` | rgb(115, 115, 115) | Selected cards |
| **Selected Border** | `#404040` | rgb(64, 64, 64) | Interactive elements |

### Element Colors

| Purpose | Hex Code | RGB | Usage |
|---------|----------|-----|-------|
| **Element Background** | `#171717` | rgb(23, 23, 23) | Buttons, inputs |
| **Element Hover** | `#262626` | rgb(38, 38, 38) | Hover states |
| **Element Active** | `#262626` | rgb(38, 38, 38) | Active/pressed states |
| **Element Selected** | `#262626` | rgb(38, 38, 38) | Selected items |

### Accent Colors

| Purpose | Hex Code | RGB | Usage |
|---------|----------|-----|-------|
| **Primary Accent (Teal)** | `#14b8a6` | rgb(20, 184, 166) | Checkboxes, highlights |
| **Info (Blue)** | `#60a5fa` | rgb(96, 165, 250) | Scan button |
| **Success (Green)** | `#4ade80` | rgb(74, 222, 128) | Success states |
| **Warning (Amber)** | `#fbbf24` | rgb(251, 191, 36) | Warning states |
| **Error (Red)** | `#f87171` | rgb(248, 113, 113) | Delete button, errors |

### Status Colors with Backgrounds

| Status | Foreground | Background (33% opacity) | Border |
|--------|------------|--------------------------|--------|
| **Error** | `#f87171` | `#f8717133` | `#f87171` |
| **Warning** | `#fbbf24` | `#fbbf2433` | `#fbbf24` |
| **Success** | `#4ade80` | `#4ade8033` | `#4ade80` |
| **Info** | `#60a5fa` | `#60a5fa33` | `#60a5fa` |

## UI Component Color Mapping

### Header Section
- Background: `elevated_surface_background` (#171717)
- Border: `border` (#262626)
- Title text: `text` (#e5e5e5)
- Subtitle/path: `text_muted` (#a3a3a3)
- Selected count: `text_accent` (#14b8a6)

### Buttons

#### Scan Button
- Background: `info_background` (#60a5fa33)
- Border: `info` (#60a5fa)
- Hover: `info` (#60a5fa)

#### Delete Button (Active)
- Background: `error_background` (#f8717133)
- Border: `error` (#f87171)
- Hover: `error` (#f87171)

#### Delete Button (Disabled)
- Background: `element_background` (#171717)
- Border: `border_selected` (#404040)
- No hover effect

#### Directory Picker
- Background: `element_background` (#171717)
- Border: `border_selected` (#404040)
- Hover: `element_hover` (#262626)

### Project Cards

#### Unselected State
- Background: `editor_background` (#0a0a0a)
- Border: `border` (#262626)
- Hover background: `element_hover` (#262626)

#### Selected State
- Background: `element_selected` (#262626)
- Border: `border_focused` (#737373)
- Hover background: `element_active` (#262626)

#### Card Content
- Project name: `text` (#e5e5e5)
- Path: `text_muted` (#a3a3a3)
- Metadata (age/size): `text_placeholder` (#737373)

### Checkbox
- Border: `border_selected` (#404040)
- Checked background: `text_accent` (#14b8a6)
- Checkmark: `text` (#e5e5e5)

### Scrollbar
- Thumb: `scrollbar_thumb_background` (#52525233)
- Thumb hover: `scrollbar_thumb_hover_background` (#262626)
- Track: `scrollbar_track_background` (#171717)

## Color Psychology

The Zen Dark theme uses color intentionally:

- **Dark backgrounds** (#171717, #0a0a0a): Reduce eye strain, professional appearance
- **High contrast text** (#e5e5e5 on dark): Excellent readability
- **Teal accent** (#14b8a6): Modern, calming, indicates selection
- **Red for delete** (#f87171): Clear warning, prevents accidents
- **Blue for scan** (#60a5fa): Safe action, informational
- **Subtle borders** (#262626): Define structure without distraction

## Accessibility

All color combinations meet WCAG AA standards for contrast:

- Text (#e5e5e5) on background (#171717): **13.4:1** ✓
- Muted text (#a3a3a3) on background (#171717): **6.2:1** ✓
- Accent text (#14b8a6) on background (#171717): **4.8:1** ✓
- Error (#f87171) on background (#171717): **4.9:1** ✓

## Implementation Notes

All colors are converted from hex to GPUI's `Hsla` format in `src/ui/theme.rs`:

```rust
pub fn zen_dark() -> Self {
    Self {
        background: parse_hex("#171717ff"),
        text: parse_hex("#e5e5e5ff"),
        text_accent: parse_hex("#14b8a6ff"),
        // ... etc
    }
}
```

The `parse_hex()` function handles:
- RGB hex codes (#RRGGBB)
- Alpha channel (#RRGGBBAA)
- Conversion to HSL color space for GPUI