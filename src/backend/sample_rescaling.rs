//! Utilities for mapping floating point values to integers and back.

/// Scales all 16-bit integer values to 32-bit floating point values between -1.0 and 1.0.
pub fn i16_to_f32(s: i16) -> f32 {
    (s as f32) / (0x8000 as f32)
}

/// Scales all 32-bit integer values to 32-bit floating point values between -1.0 and 1.0.
pub fn i32_to_f32(s: i32) -> f32 {
    (s as f32) / (0x7FFFFFFF as f32)
}

/// Scales 32-bit floating point values between -1.0 and 1.0 to a 16-bit integer value.
pub fn f32_to_i16(s: f32) -> i16 {
    (s * 0x8000 as f32) as i16
}

/// Scales 32-bit floating point values between -1.0 and 1.0 to a 16-bit integer value.
#[allow(dead_code)] // Remove this once we have a function using it.
pub fn f32_to_i32(s: f32) -> i32 {
    (s * 0x7FFFFFFF as f32) as i32
}
