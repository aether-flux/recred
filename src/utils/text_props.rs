pub fn resolve_color(field_color: Option<[u8; 3]>, global_color: Option<[u8; 3]>) -> [f32; 3] {
    let color = field_color.or(global_color).unwrap_or([0, 0, 0]);  // default: Black
    [
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
    ]
}

pub fn resolve_size(field_size: Option<f32>, global_size: Option<f32>) -> f32 {
    field_size.or(global_size).unwrap_or(25.0)  // default: 25.0
}
