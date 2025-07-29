#[inline]
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn round_f32(value: f32) -> f32 {
    let int_part = if value >= 0.0 {
        (value + 0.5) as i32
    } else {
        (value - 0.5) as i32
    };
    int_part as f32
}

#[inline]
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn round_f64(value: f64) -> f64 {
    let int_part = if value >= 0.0 {
        (value + 0.5) as i64
    } else {
        (value - 0.5) as i64
    };
    int_part as f64
}
