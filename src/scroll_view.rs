use crate::animation::Motion;
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::interaction::animate_hover;
use crate::scrolling::{ScrollAxisState, ScrollConfig, compute_geometry, handle_drag};
use crate::stateful::{StateWeights, Stateful};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, AnyWidget, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Default)]
pub struct ScrollState {
    pub x: ScrollAxisState,
    pub y: ScrollAxisState,
}

#[derive(Clone)]
pub struct ScrollViewStyle {
    pub fill: Stateful<Option<Fill>>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub background_path: Stateful<Path>,
    pub track_fill: Stateful<Fill>,
    pub thumb_fill: Stateful<Fill>,
    pub scrollbar: Stateful<ScrollConfig>,
    pub always_show_scrollbars: bool,
    pub scroll_speed: f32,
    pub clip_content: bool,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub track_path: Stateful<Path>,
    pub hover_motion: f32,
    pub cursor: Stateful<CursorIcon>,
}

pub fn pill_path() -> Path {
    Path::from_fn(|position, size| {
        MeasurablePath::rect(position, size, [size[0].min(size[1]) * 0.5; 4])
    })
}

impl ScrollViewStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        let (thumb, active) = if theme.is_dark {
            (Color::WHITE.with_alpha(0.35), Color::WHITE.with_alpha(0.65))
        } else {
            (
                theme.text_secondary.with_alpha(0.60),
                theme.text_primary.with_alpha(0.85),
            )
        };
        ScrollViewStyle {
            fill: Stateful::new(None),
            border_width: Stateful::new(0.0),
            border_color: Stateful::new(Color::TRANSPARENT),
            background_path: Stateful::new(Path::rect([0.0; 4])),
            track_fill: Stateful::new(Fill::Solid(Color::TRANSPARENT)),
            thumb_fill: Stateful::new(Fill::Solid(thumb))
                .hovered(Fill::Solid(active))
                .pressed(Fill::Solid(active)),
            scrollbar: Stateful::new(ScrollConfig::default()),
            always_show_scrollbars: false,
            scroll_speed: 1.0,
            clip_content: true,
            sharp: false,
            path: Stateful::new(pill_path()),
            track_path: Stateful::new(pill_path()),
            hover_motion: Motion::SNAPPY,
            cursor: Stateful::new(CursorIcon::Pointer),
        }
    }

    pub fn panel(theme: &Theme) -> Self {
        ScrollViewStyle {
            fill: Stateful::new(Some(Fill::Solid(theme.surface))),
            border_width: Stateful::new(1.0),
            border_color: Stateful::new(theme.border),
            background_path: Stateful::new(Path::rect([theme.metrics.radius_md; 4])),
            track_fill: Stateful::new(Fill::Solid(theme.surface_subtle)),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for ScrollViewStyle {
    fn default() -> Self {
        ScrollViewStyle::from_theme(&Theme::default())
    }
}

pub struct ScrollView<'a> {
    id: String,
    width: f32,
    height: f32,
    padding: [f32; 2],
    style: Option<ScrollViewStyle>,
    child: Box<dyn AnyWidget + 'a>,
    default_offset: [f32; 2],
}

impl<'a> ScrollView<'a> {
    pub const DEFAULT_WIDTH: f32 = 300.0;
    pub const DEFAULT_HEIGHT: f32 = 200.0;

    pub fn new(child: &'a mut impl Measurable) -> Self {
        ScrollView {
            id: generate_uuid(),
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            padding: [0.0, 0.0],
            style: None,
            child: Box::new(child),
            default_offset: [0.0; 2],
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
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

    fn resolved_style(&self, theme: &Theme) -> ScrollViewStyle {
        self.style
            .clone()
            .unwrap_or_else(|| ScrollViewStyle::from_theme(theme))
    }

    pub fn default_offset(mut self, offset: [f32; 2]) -> Self {
        self.default_offset = offset;
        self
    }

    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        let size = self.measure(ui);
        Measurable::arrange(self, position, size, ui);
    }

    pub fn offset(&self, ui: &mut Ui) -> [f32; 2] {
        let state = self.state(ui);
        [state.x.offset, state.y.offset]
    }
    pub fn set_offset(&mut self, ui: &mut Ui, offset: [f32; 2]) {
        let mut state = self.take_state(ui);
        state.x.offset = offset[0];
        state.y.offset = offset[1];
        self.put_state(ui, state);
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
        self.check_id();
        let position = [position[0] + self.padding[0], position[1] + self.padding[1]];
        let size = [
            (size[0] - self.padding[0] * 2.0).max(0.0),
            (size[1] - self.padding[1] * 2.0).max(0.0),
        ];
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        let hovered = MeasurablePath::rect(position, size, [0.0; 4]).contains(ui.mouse_position());
        let hover_id = format!("{}__hover", self.id);
        let weights =
            StateWeights::IDLE.hovered(animate_hover(ui, &hover_id, hovered, style.hover_motion));
        let config = style.scrollbar.resolve(weights);

        if let Some(fill) = style.fill.resolve(weights) {
            ui.draw_shape(
                (style.background_path.resolve(weights))(position, size),
                fill,
                style.border_width.resolve(weights),
                style.border_color.resolve(weights),
                0.0,
                style.sharp,
                0.0,
            );
        }
        let mut state = ui.take_widget_state_or(&self.id, self.initial_state());

        // wheel is applied after child arrange so a nested scroll view can consume it first
        let content_size = self.child.measure(ui);

        let track_length_y = size[1] - config.padding * 2.0;
        let geometry_y = compute_geometry(
            size[1],
            content_size[1],
            track_length_y,
            state.y.offset,
            &config,
        );

        let track_x = position[0] + size[0] - config.thickness - config.padding;
        let track_rect_position_y = [track_x, position[1]];
        let track_rect_size_y = [config.thickness + config.padding, size[1]];
        let track_hovered_y =
            MeasurablePath::rect(track_rect_position_y, track_rect_size_y, [0.0; 4])
                .contains(ui.mouse_position());
        // linger countdown should start from pointer leave not last scroll
        if track_hovered_y {
            state.y.mark_activity();
        }

        let thumb_position_y = [
            track_x,
            position[1] + config.padding + geometry_y.thumb_position_along_track,
        ];
        let thumb_size_y = [config.thickness, geometry_y.thumb_size];
        let thumb_hovered_y = MeasurablePath::rect(thumb_position_y, thumb_size_y, [0.0; 4])
            .contains(ui.mouse_position());

        if let Some(new_offset) = handle_drag(
            &mut state.y,
            position[1] + config.padding,
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

        let track_length_x = size[0] - config.padding * 2.0;
        let geometry_x = compute_geometry(
            size[0],
            content_size[0],
            track_length_x,
            state.x.offset,
            &config,
        );

        let track_y = position[1] + size[1] - config.thickness - config.padding;
        let track_rect_position_x = [position[0], track_y];
        let track_rect_size_x = [size[0], config.thickness + config.padding];
        let track_hovered_x =
            MeasurablePath::rect(track_rect_position_x, track_rect_size_x, [0.0; 4])
                .contains(ui.mouse_position());
        if track_hovered_x {
            state.x.mark_activity();
        }

        let thumb_position_x = [
            position[0] + config.padding + geometry_x.thumb_position_along_track,
            track_y,
        ];
        let thumb_size_x = [geometry_x.thumb_size, config.thickness];
        let thumb_hovered_x = MeasurablePath::rect(thumb_position_x, thumb_size_x, [0.0; 4])
            .contains(ui.mouse_position());

        if let Some(new_offset) = handle_drag(
            &mut state.x,
            position[0] + config.padding,
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
            ui.set_cursor_icon(style.cursor.resolve(thumb_weights(
                state.x.dragging || state.y.dragging,
                thumb_hovered_x || thumb_hovered_y,
            )));
        }

        state.x.offset = state.x.offset.clamp(0.0, geometry_x.max_scroll);
        state.y.offset = state.y.offset.clamp(0.0, geometry_y.max_scroll);

        let geometry_y_final = compute_geometry(
            size[1],
            content_size[1],
            track_length_y,
            state.y.offset,
            &config,
        );
        let thumb_position_y = [
            track_x,
            position[1] + config.padding + geometry_y_final.thumb_position_along_track,
        ];

        let geometry_x_final = compute_geometry(
            size[0],
            content_size[0],
            track_length_x,
            state.x.offset,
            &config,
        );
        let thumb_position_x = [
            position[0] + config.padding + geometry_x_final.thumb_position_along_track,
            track_y,
        ];

        let has_scroll_y = geometry_y.max_scroll > 0.0;
        let has_scroll_x = geometry_x.max_scroll > 0.0;
        let active_y = style.always_show_scrollbars
            || track_hovered_y
            || state.y.dragging
            || state.y.recently_active(&config);
        let active_x = style.always_show_scrollbars
            || track_hovered_x
            || state.x.dragging
            || state.x.recently_active(&config);

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
        if style.clip_content {
            ui.push_clip(clip_rect);
        }

        let child_position = [position[0] - state.x.offset, position[1] - state.y.offset];
        self.child.arrange(child_position, content_size, ui);

        if style.clip_content {
            ui.pop_clip();
        }

        // claimed after nested children so only the topmost hovered view scrolls
        if !ui.scroll_consumed()
            && hovered
            && (ui.scroll_delta_x() != 0.0 || ui.scroll_delta_y() != 0.0)
        {
            let speed = style.scroll_speed;
            if ui.shift_held() {
                state.x.offset -= ui.scroll_delta_y() * speed;
            } else {
                state.x.offset -= ui.scroll_delta_x() * speed;
                state.y.offset -= ui.scroll_delta_y() * speed;
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
            let track = track_weights(track_hovered_y);
            ui.draw_shape(
                (style.track_path.resolve(track))(
                    [track_x, position[1] + config.padding],
                    [config.thickness, size[1] - config.padding * 2.0],
                ),
                style.track_fill.resolve(track),
                0.0,
                Color::TRANSPARENT,
                0.0,
                style.sharp,
                0.0,
            );
            let thumb = thumb_weights(state.y.dragging, thumb_hovered_y);
            let thumb_fill = style.thumb_fill.resolve(thumb);
            ui.draw_shape(
                (style.path.resolve(thumb))(
                    thumb_position_y,
                    [config.thickness, geometry_y_final.thumb_size],
                ),
                thumb_fill,
                0.0,
                Color::TRANSPARENT,
                0.0,
                style.sharp,
                0.0,
            );
        }

        if show_x {
            let track = track_weights(track_hovered_x);
            ui.draw_shape(
                (style.track_path.resolve(track))(
                    [position[0] + config.padding, track_y],
                    [size[0] - config.padding * 2.0, config.thickness],
                ),
                style.track_fill.resolve(track),
                0.0,
                Color::TRANSPARENT,
                0.0,
                style.sharp,
                0.0,
            );
            let thumb = thumb_weights(state.x.dragging, thumb_hovered_x);
            let thumb_fill = style.thumb_fill.resolve(thumb);
            ui.draw_shape(
                (style.path.resolve(thumb))(
                    thumb_position_x,
                    [geometry_x_final.thumb_size, config.thickness],
                ),
                thumb_fill,
                0.0,
                Color::TRANSPARENT,
                0.0,
                style.sharp,
                0.0,
            );
        }

        ui.put_widget_state(&self.id, state);
    }
}

fn track_weights(hovered: bool) -> StateWeights {
    StateWeights::IDLE.hovered(StateWeights::when(hovered))
}

fn thumb_weights(dragging: bool, hovered: bool) -> StateWeights {
    StateWeights::IDLE
        .hovered(StateWeights::when(hovered))
        .pressed(StateWeights::when(dragging))
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
