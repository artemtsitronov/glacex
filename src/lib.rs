//! Experimental GPU-rendered UI toolkit built with `wgpu`, `winit`, and `taffy`.
//!
//! The crate exposes a small custom widget system centered around three core pieces:
//! - [`App`], which owns the native window and event loop
//! - [`Ui`], which stores frame state, input state, and per-widget persistent state
//! - [`Widget`], implemented by widgets that render themselves each frame
//!
//! Widgets are redrawn every frame, while interactive state is kept inside [`Ui`] and
//! keyed by stable widget IDs. The repository also includes a runnable demo in
//! `src/main.rs`.

pub mod alignment;
pub mod button;
pub mod checkbox;
pub mod color;
pub mod container;
pub mod geometry;
pub mod interaction;
pub mod label;
pub mod layout;
pub mod painter;
pub mod radio_button;
pub mod scroll_view;
pub mod scrolling;
pub mod shapes;
pub mod text_area;
pub mod text_edit;
pub mod text_input;
pub mod theme;
pub mod ui;
pub mod widget;

pub use alignment::*;
pub use button::*;
pub use checkbox::*;
pub use color::*;
pub use container::*;
pub use geometry::*;
pub use interaction::*;
pub use label::*;
pub use layout::*;
pub use radio_button::*;
pub use scroll_view::*;
pub use scrolling::*;
pub use text_area::*;
pub use text_edit::*;
pub use text_input::*;
pub use theme::*;
pub use ui::*;
pub use widget::*;

use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey, PhysicalKey};
use winit::window::{Window, WindowId};

type UpdateFn<W> = Box<dyn FnMut(&mut W)>;

/// Entry point for a client app. Call `App::new(root_widget).run()`.
/// `root` is built once and lives for the whole app; it's re-drawn every
/// frame. An optional `update` callback runs first each frame, with
/// mutable access to `root`, so it can react to input (e.g. button clicks)
/// before drawing happens.
pub struct App<W: Widget> {
    window: Option<Arc<Window>>,
    ui: Option<Ui>,
    root: W,
    update_fn: Option<UpdateFn<W>>,
}

impl<W: Widget> App<W> {
    pub fn new(root: W) -> Self {
        App {
            window: None,
            ui: None,
            root,
            update_fn: None,
        }
    }

    /// Registers a callback that runs once per frame, before drawing.
    /// Use `Column::get_mut`/`Row::get_mut` inside it to pull concrete
    /// widgets (e.g. a `Button`) back out of the tree and check state.
    pub fn update(mut self, f: impl FnMut(&mut W) + 'static) -> Self {
        self.update_fn = Some(Box::new(f));
        self
    }

    pub fn run(self) {
        let _ = env_logger::try_init();

        let event_loop = match EventLoop::new() {
            Ok(event_loop) => event_loop,
            Err(error) => {
                eprintln!("failed to create event loop: {error}");
                return;
            }
        };

        let mut app = self;
        if let Err(error) = event_loop.run_app(&mut app) {
            eprintln!("application event loop exited with an error: {error}");
        }
    }
}

impl<W: Widget> ApplicationHandler for App<W> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match event_loop.create_window(Window::default_attributes()) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                eprintln!("failed to create window: {error}");
                event_loop.exit();
                return;
            }
        };

        self.ui = Some(pollster::block_on(Ui::new(window.clone())));
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let ui = match self.ui.as_mut() {
            Some(ui) => ui,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                ui.begin_frame();
                if let Some(update_fn) = self.update_fn.as_mut() {
                    update_fn(&mut self.root);
                }
                let _ = self.root.ui(ui);

                if ui.key_pressed(Key::Named(NamedKey::Tab)) {
                    ui.advance_focus(ui.shift_held());
                } else if ui.mouse_pressed_this_frame() && !ui.focus_requested_this_frame() {
                    ui.clear_focus();
                }

                ui.render();
                ui.end_frame();
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::Resized(size) => {
                ui.resize(size.width, size.height);
            }

            WindowEvent::CursorMoved { position, .. } => {
                ui.update_mouse_position(position.x, position.y);
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    ui.set_mouse_pressed(state == ElementState::Pressed);
                    ui.set_mouse_released(state == ElementState::Released);
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x * 20.0, y * 20.0),
                    winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                ui.set_scroll_delta_x(dx);
                ui.set_scroll_delta_y(dy);
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let Some(text) = &event.text {
                        ui.push_typed_text(text);
                    }
                    ui.mark_key_pressed(event.logical_key.clone());
                    if let PhysicalKey::Code(_) = event.physical_key {
                        ui.mark_physical_key_pressed(event.physical_key);
                    }
                }
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                let state = modifiers.state();
                ui.set_shift_held(state.shift_key());
                ui.set_ctrl_held(modifiers.state().control_key());
            }

            _ => {}
        }
    }
}
