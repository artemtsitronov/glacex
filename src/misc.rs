// miscellanous stuff, that was abandoned, and didn't fit into anywhere.
// Tragic.

pub fn center_text_in(
    position: [f32; 2],
    size: [f32; 2],
    text_width: f32,
    line_height: f32,
) -> [f32; 2] {
    [
        position[0] + (size[0] - text_width) / 2.0,
        position[1] + (size[1] - line_height) / 2.0,
    ]
}
