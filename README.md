# glacex

![demo](screenshots/screenshot_demo.png)

GPU-Rendered UI library — built from scratch in Rust [WIP]

## About

glacex is a GPU-accelerated, immediate-mode UI library built entirely 
from scratch using `winit` and `wgpu`, with no dependency on existing 
UI frameworks. Currently in early development.

## Status

⚠️ This project is under active development and not yet ready for 
production use. APIs are unstable and subject to change.

## Features (so far)

- Instanced GPU rendering
- SDF rounded-corner shapes
- Text rendering via `glyphon`
- Hit-testing and interaction system
- Layout system (Row/Column)
- A filling system (Solid, Gradient)
- Widgets: button, checkbox, radio button, label, text input, 
  text area, scroll view, container
