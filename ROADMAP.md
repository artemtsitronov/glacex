# Glacex Roadmap

## Goals

Build a fast, cross-platform GPU-rendered UI toolkit in Rust with an immediate-mode API, clean defaults, and a widget set suitable for daily application development.

## Release Milestones

| Version | Focus | Deliverables |
|---|---|---|
| **v0.1.0** | Core baseline | Initial widgets, taffy flexbox, SDF rendering pipeline, theming. |
| **v0.2.0** | Engine stability | Layout refinements, clip stacks, secondary mouse input, cursor switching, tooltip overlays. |
| **v0.3.0** | Widget completeness | TreeView, Table/ListView, MenuBar, Dropdown/ComboBox, ModalDialog, ColorPicker. |
| **v0.4.0** | Platform integration | High-DPI handling, multi-window support, native file dialogs, system tray integration. |
| **v0.5.0** | Runtime features | Property transitions, spring animations, reactive state helpers, dark/light theme switching. |
| **v0.6.0** | Accessibility & i18n | Keyboard focus traversal, screen reader hooks, IME input composition, RTL text layout. |
| **v1.0.0** | Stable release | Stable API contracts, automated visual regression test suite, complete documentation, benchmark suite. |

## Maintenance & Releases

- **Versioning**: Follows Semantic Versioning 2.0.0.
- **CI**: Automated formatting, Clippy linting, and multi-target compilation on every PR.
- **Auditing**: Regular dependency audits via `cargo audit`.
