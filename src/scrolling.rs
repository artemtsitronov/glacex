// scrolling.rs
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct ScrollConfig {
    pub padding: f32,
    pub thickness: f32,
    pub min_thumb: f32,
    pub linger_seconds: f32,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        ScrollConfig {
            padding: 2.0,
            thickness: 5.0,
            min_thumb: 20.0,
            linger_seconds: 0.8,
        }
    }
}

pub struct ScrollAxisState {
    pub offset: f32,
    pub dragging: bool,
    pub drag_grab_offset: f32,
    pub last_activity: Instant,
}

impl Default for ScrollAxisState {
    fn default() -> Self {
        ScrollAxisState {
            offset: 0.0,
            dragging: false,
            drag_grab_offset: 0.0,
            last_activity: Instant::now(),
        }
    }
}

impl ScrollAxisState {
    pub fn mark_activity(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn recently_active(&self, config: &ScrollConfig) -> bool {
        self.last_activity.elapsed().as_secs_f32() < config.linger_seconds
    }
}

pub struct ScrollGeometry {
    pub max_scroll: f32,
    pub thumb_size: f32,
    pub thumb_travel: f32,
    pub thumb_position_along_track: f32, // 0.0 = start of track, grows with thumb_travel
}

pub fn compute_geometry(
    box_size: f32,
    content_size: f32,
    track_length: f32,
    offset: f32,
    config: &ScrollConfig,
) -> ScrollGeometry {
    if content_size <= 0.0 {
        return ScrollGeometry {
            max_scroll: 0.0,
            thumb_size: track_length,
            thumb_travel: 0.0,
            thumb_position_along_track: 0.0,
        };
    }

    let max_scroll = (content_size - box_size).max(0.0);
    let thumb_size = (box_size / content_size * track_length)
        .max(config.min_thumb)
        .min(track_length);
    let thumb_travel = (track_length - thumb_size).max(0.0);
    let thumb_position_along_track = if max_scroll > 0.0 {
        (offset / max_scroll) * thumb_travel
    } else {
        0.0
    };
    ScrollGeometry {
        max_scroll,
        thumb_size,
        thumb_travel,
        thumb_position_along_track,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_drag(
    axis_state: &mut ScrollAxisState,
    thumb_track_start: f32, // position[axis] + padding, in screen space
    mouse_pos_along_axis: f32,
    thumb_start_in_screen: f32, // where the thumb currently starts, in screen space
    mouse_pressed_this_frame: bool,
    mouse_pressed: bool,
    thumb_hovered: bool,
    geometry: &ScrollGeometry,
) -> Option<f32> {
    if mouse_pressed_this_frame && thumb_hovered {
        axis_state.dragging = true;
        axis_state.drag_grab_offset = mouse_pos_along_axis - thumb_start_in_screen;
    }
    if !mouse_pressed {
        axis_state.dragging = false;
    }
    if axis_state.dragging && geometry.thumb_travel > 0.0 {
        let raw = mouse_pos_along_axis - thumb_track_start - axis_state.drag_grab_offset;
        let ratio = (raw / geometry.thumb_travel).clamp(0.0, 1.0);
        Some(ratio * geometry.max_scroll)
    } else {
        None
    }
}
