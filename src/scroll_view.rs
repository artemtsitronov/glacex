use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::contains;
use crate::scrolling::{ScrollAxisState, ScrollConfig, compute_geometry, handle_drag};
use crate::ui::Ui;
use crate::widget::{Accessible, AnyWidget, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Default)]
pub struct ScrollState {
    pub x: ScrollAxisState,
    pub y: ScrollAxisState,
}

#[derive(Debug, Clone)]
pub struct ScrollViewStyle {
    pub thumb_fill: Fill,
    pub thumb_dragging_fill: Fill,
    pub thumb_corner_radius: f32,
}

impl Default for ScrollViewStyle {
    fn default() -> Self {
        ScrollViewStyle {
            thumb_fill: Fill::Solid(Color::rgb(113, 113, 122).with_alpha(0.5)),
            thumb_dragging_fill: Fill::Solid(Color::rgb(82, 82, 91).with_alpha(0.75)),
            thumb_corner_radius: ScrollConfig::default().thickness / 2.0,
        }
    }
}

pub struct ScrollView<'a> {
    id: String,
    width: f32,
    height: f32,
    padding: [f32; 2],
    config: ScrollConfig,
    style: Option<ScrollViewStyle>,
    child: Box<dyn AnyWidget + 'a>,
    default_offset: [f32; 2],
}

impl<'a> ScrollView<'a> {
    pub const DEFAULT_WIDTH: f32 = 300.0;
    pub const DEFAULT_HEIGHT: f32 = 200.0;

    pub fn new(id: impl Into<String>, child: &'a mut impl Measurable) -> Self {
        ScrollView {
            id: id.into(),
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            padding: [0.0, 0.0],
            config: ScrollConfig::default(),
            style: None,
            child: Box::new(child),
            default_offset: [0.0; 2],
        }
    }

    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = padding;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = size[0];
        self.height = size[1];
        self
    }

    pub fn style(mut self, style: ScrollViewStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<ScrollViewStyle>) {
        self.style = style;
    }

    pub fn default_offset(mut self, offset: [f32; 2]) -> Self {
        self.default_offset = offset;
        self
    }

    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        let size = self.measure(ui);
        Measurable::arrange(self, position, size, ui);
    }
}

impl<'a> Widget for ScrollView<'a> {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        self.arrange_at([0.0, 0.0], ui);
    }
}

impl<'a> Measurable for ScrollView<'a> {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width, self.height]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let position = [position[0] + self.padding[0], position[1] + self.padding[1]];
        let size = [
            (size[0] - self.padding[0] * 2.0).max(0.0),
            (size[1] - self.padding[1] * 2.0).max(0.0),
        ];
        let style = self.style.clone().unwrap_or_else(|| {
            let theme = *ui.theme();
            if theme.is_dark {
                ScrollViewStyle {
                    thumb_fill: Fill::Solid(Color::WHITE.with_alpha(0.35)),
                    thumb_dragging_fill: Fill::Solid(Color::WHITE.with_alpha(0.65)),
                    thumb_corner_radius: ScrollConfig::default().thickness / 2.0,
                }
            } else {
                ScrollViewStyle {
                    thumb_fill: Fill::Solid(theme.text_secondary.with_alpha(0.60)),
                    thumb_dragging_fill: Fill::Solid(theme.text_primary.with_alpha(0.85)),
                    thumb_corner_radius: ScrollConfig::default().thickness / 2.0,
                }
            }
        });

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        let hovered = contains(position, size, 0.0, ui.mouse_position());
        let mut state = ui.take_widget_state_or(&self.id, self.initial_state());

        // Applying the wheel event is deferred until after `self.child` has
        // been arranged below — if the child is (or contains) a nested
        // ScrollView, its own arrange() call claims the event first via
        // `ui.consume_scroll()`, so a wheel tick over a nested scroll area
        // only moves that inner view instead of every hovered ancestor too.

        let content_size = self.child.measure(ui);

        let track_length_y = size[1] - self.config.padding * 2.0;
        let geometry_y = compute_geometry(
            size[1],
            content_size[1],
            track_length_y,
            state.y.offset,
            &self.config,
        );

        let track_x = position[0] + size[0] - self.config.thickness - self.config.padding;
        let track_rect_position_y = [track_x, position[1]];
        let track_rect_size_y = [self.config.thickness + self.config.padding, size[1]];
        let track_hovered_y = contains(
            track_rect_position_y,
            track_rect_size_y,
            0.0,
            ui.mouse_position(),
        );
        // Keep resetting the "last activity" clock while the pointer is on
        // the track, so the linger countdown only starts once it actually
        // leaves — not from whatever scroll/drag last happened.
        if track_hovered_y {
            state.y.mark_activity();
        }

        let thumb_position_y = [
            track_x,
            position[1] + self.config.padding + geometry_y.thumb_position_along_track,
        ];
        let thumb_size_y = [self.config.thickness, geometry_y.thumb_size];
        let thumb_hovered_y = contains(thumb_position_y, thumb_size_y, 0.0, ui.mouse_position());

        if let Some(new_offset) = handle_drag(
            &mut state.y,
            position[1] + self.config.padding,
            ui.mouse_position()[1],
            thumb_position_y[1],
            ui.mouse_pressed_this_frame(),
            ui.mouse_pressed(),
            thumb_hovered_y,
            &geometry_y,
        ) {
            state.y.offset = new_offset;
            state.y.mark_activity();
        }

        let track_length_x = size[0] - self.config.padding * 2.0;
        let geometry_x = compute_geometry(
            size[0],
            content_size[0],
            track_length_x,
            state.x.offset,
            &self.config,
        );

        let track_y = position[1] + size[1] - self.config.thickness - self.config.padding;
        let track_rect_position_x = [position[0], track_y];
        let track_rect_size_x = [size[0], self.config.thickness + self.config.padding];
        let track_hovered_x = contains(
            track_rect_position_x,
            track_rect_size_x,
            0.0,
            ui.mouse_position(),
        );
        if track_hovered_x {
            state.x.mark_activity();
        }

        let thumb_position_x = [
            position[0] + self.config.padding + geometry_x.thumb_position_along_track,
            track_y,
        ];
        let thumb_size_x = [geometry_x.thumb_size, self.config.thickness];
        let thumb_hovered_x = contains(thumb_position_x, thumb_size_x, 0.0, ui.mouse_position());

        if let Some(new_offset) = handle_drag(
            &mut state.x,
            position[0] + self.config.padding,
            ui.mouse_position()[0],
            thumb_position_x[0],
            ui.mouse_pressed_this_frame(),
            ui.mouse_pressed(),
            thumb_hovered_x,
            &geometry_x,
        ) {
            state.x.offset = new_offset;
            state.x.mark_activity();
        }

        if thumb_hovered_x || thumb_hovered_y || state.x.dragging || state.y.dragging {
            ui.set_cursor_icon(CursorIcon::Pointer);
        }

        state.x.offset = state.x.offset.clamp(0.0, geometry_x.max_scroll);
        state.y.offset = state.y.offset.clamp(0.0, geometry_y.max_scroll);

        let geometry_y_final = compute_geometry(
            size[1],
            content_size[1],
            track_length_y,
            state.y.offset,
            &self.config,
        );
        let thumb_position_y = [
            track_x,
            position[1] + self.config.padding + geometry_y_final.thumb_position_along_track,
        ];

        let geometry_x_final = compute_geometry(
            size[0],
            content_size[0],
            track_length_x,
            state.x.offset,
            &self.config,
        );
        let thumb_position_x = [
            position[0] + self.config.padding + geometry_x_final.thumb_position_along_track,
            track_y,
        ];

        let has_scroll_y = geometry_y.max_scroll > 0.0;
        let has_scroll_x = geometry_x.max_scroll > 0.0;
        let active_y = track_hovered_y || state.y.dragging || state.y.recently_active(&self.config);
        let active_x = track_hovered_x || state.x.dragging || state.x.recently_active(&self.config);

        // Thumb only shows up once the pointer actually touches the track
        // (or while dragging), then lingers for `config.linger_seconds`
        // after the pointer leaves before hiding again.
        let show_y = has_scroll_y && active_y;
        let show_x = has_scroll_x && active_x;

        if show_y {
            ui.push_input_block([
                track_rect_position_y[0],
                track_rect_position_y[1],
                track_rect_position_y[0] + track_rect_size_y[0],
                track_rect_position_y[1] + track_rect_size_y[1],
            ]);
        }
        if show_x {
            ui.push_input_block([
                track_rect_position_x[0],
                track_rect_position_x[1],
                track_rect_position_x[0] + track_rect_size_x[0],
                track_rect_position_x[1] + track_rect_size_x[1],
            ]);
        }

        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];
        ui.push_clip(clip_rect);

        let child_position = [position[0] - state.x.offset, position[1] - state.y.offset];
        self.child.arrange(child_position, content_size, ui);

        ui.pop_clip();

        // Claim the wheel event now, after any nested scrollable inside
        // `self.child` has had first crack at it. `state.offset` changing
        // here lands visually next frame — an imperceptible one-frame lag
        // in exchange for a wheel tick only ever moving the topmost/most
        // specific hovered scroll area.
        if !ui.scroll_consumed()
            && hovered
            && (ui.scroll_delta_x() != 0.0 || ui.scroll_delta_y() != 0.0)
        {
            if ui.shift_held() {
                state.x.offset -= ui.scroll_delta_y();
            } else {
                state.x.offset -= ui.scroll_delta_x();
                state.y.offset -= ui.scroll_delta_y();
            }
            state.x.offset = state.x.offset.clamp(0.0, geometry_x.max_scroll);
            state.y.offset = state.y.offset.clamp(0.0, geometry_y.max_scroll);
            state.x.mark_activity();
            state.y.mark_activity();
            ui.consume_scroll();
        }

        if show_x {
            ui.pop_input_block();
        }
        if show_y {
            ui.pop_input_block();
        }

        if show_y {
            let thumb_fill = if state.y.dragging || active_y {
                style.thumb_dragging_fill.clone()
            } else {
                style.thumb_fill.clone()
            };
            ui.draw_rect(
                thumb_position_y,
                [self.config.thickness, geometry_y_final.thumb_size],
                thumb_fill,
                style.thumb_corner_radius,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        if show_x {
            let thumb_fill = if state.x.dragging || active_x {
                style.thumb_dragging_fill
            } else {
                style.thumb_fill
            };
            ui.draw_rect(
                thumb_position_x,
                [geometry_x_final.thumb_size, self.config.thickness],
                thumb_fill,
                style.thumb_corner_radius,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        ui.put_widget_state(&self.id, state);
    }
}

impl<'a> StatefulWidget for ScrollView<'a> {
    type State = ScrollState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> ScrollState {
        ScrollState {
            x: ScrollAxisState {
                offset: self.default_offset[0],
                ..Default::default()
            },
            y: ScrollAxisState {
                offset: self.default_offset[1],
                ..Default::default()
            },
        }
    }
}

impl<'a> Accessible for ScrollView<'a> {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::ScrollView
    }
}
